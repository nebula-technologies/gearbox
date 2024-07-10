import subprocess
from bs4 import BeautifulSoup
import os
import re


def run_cargo_readme(args=None):
    """Runs `cargo readme` with optional arguments and returns its output."""
    cmd = ['cargo', 'readme']
    if args:
        cmd.extend(args)
    try:
        result = subprocess.run(cmd, capture_output=True, text=True, check=True)
        return result.stdout
    except subprocess.CalledProcessError as e:
        print(f"An error occurred while running 'cargo readme': {e}")
        print(f"Error output: {e.stderr}")
        return None


def write_to_file(file_path, content):
    """Writes the given content to the specified file."""
    with open(file_path, 'w') as file:
        file.write(content)


def read_file(file_path):
    """Reads the content of the specified file and returns it."""
    with open(file_path, 'r') as file:
        return file.read()


def generate_header():
    """Generates the markdown table header for the report."""
    return ("| File | Coverage Bar | Line Coverage | Lines Covered | Lines Total |\n"
            "|------|--------------|---------------|---------------|-------------|\n")


def generate_row(file_name, line_coverage, lines_covered, lines_total):
    """Generates a markdown table row for each record."""
    coverage_bar = generate_bar(line_coverage)
    return f"| {file_name} | {coverage_bar} | {line_coverage}% | {lines_covered} | {lines_total} |\n"


def generate_bar(coverage):
    """Generates a coverage bar as an image."""
    coverage_int = int(round(coverage))
    return f"![](https://geps.dev/progress/{coverage_int})"


def parse_html_coverage_report(html_file):
    """Parses the HTML coverage report and generates markdown rows."""
    with open(html_file, "r") as file:
        soup = BeautifulSoup(file, "html.parser")

    rows = soup.find_all("tr")
    markdown_report = generate_header()

    for row in rows[1:]:
        cells = row.find_all("td")
        if len(cells) >= 6:
            file_name = row.find("th").text.strip()
            line_coverage = float(cells[1].text.strip().strip('%'))
            lines_covered, lines_total = map(int, cells[2].text.strip().split('/'))
            markdown_report += generate_row(file_name, line_coverage, lines_covered, lines_total)

    return markdown_report


def replace_test_status(readme_content, test_status_content):
    """Replaces the placeholder with the test status content in the README."""
    return readme_content.replace('[See Test Status](./TEST_STATUS.md)', test_status_content)


def replace_docs_links(readme_content):
    """Replaces [docs: sometext](./filename.md) or [docs: sometext](./filename.rs) with the file content or generated markdown."""
    pattern = r'\(\[docs: ([^\]]+)\]\(\./([^\)]+\.(md|rs))\)\)'
    matches = re.findall(pattern, readme_content)

    for match in matches:
        file_name = match[1]
        doc_text = match[0]
        if os.path.exists(file_name):
            if file_name.endswith('.rs'):
                print(f"Generating markdown content for: {file_name}")
                # Run `cargo readme` command for .rs files
                file_content = run_cargo_readme(
                    ['-i', file_name, '--no-template', '--no-license', '--no-badges', '--no-title'])
            else:
                print(f"Generating markdown content for: {file_name}")
                # Read the content of .md files
                file_content = read_file(file_name)

            if file_content:
                print(f"Replacing {file_name} with generated markdown content.")
                readme_content = readme_content.replace(f'([docs: {doc_text}](./{file_name}))',
                                                        file_content)
        else:
            print(f"Referenced file not found: {file_name}")

    return readme_content


def main():
    readme_content = run_cargo_readme()
    if readme_content is None:
        return

    write_to_file('README.md', readme_content)
    html_file = ".artifacts/coverage/html/index.html"
    if not os.path.exists(html_file):
        print(f"HTML report file not found: {html_file}")
        return

    markdown_report = parse_html_coverage_report(html_file)
    readme_content = replace_test_status(readme_content, markdown_report)
    readme_content = replace_docs_links(readme_content)

    write_to_file('README.md', readme_content)
    write_to_file("TEST_STATUS.md", markdown_report)
    print("README.md and TEST_STATUS.md have been generated successfully.")


if __name__ == "__main__":
    main()
