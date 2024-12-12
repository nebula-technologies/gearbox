use crate::error::tracer::DynTracerError;
use crate::time::duration::Duration;
use spin::lock_api::{RwLock, RwLockReadGuard, RwLockWriteGuard};

#[derive(Debug, Default)]
pub struct CacheWrapper(RwLock<Cache>);

impl CacheWrapper {
    #[allow(unused)]
    pub fn new() -> Self {
        Self(RwLock::new(Cache::default()))
    }

    pub fn clear(&self) {
        (Ok(self.0.write()) as Result<RwLockWriteGuard<'_, Cache>, DynTracerError>)
            .map(|mut t| {
                let t = &mut *t;
                t.year = None;
                t.month = None;
                t.day = None;
                t.hour = None;
                t.minute = None;
                t.second = None;
                t.nanosecond = None;
                t.day_of_week = None;
                t.week = None;
                t.day_of_year = None;
            })
            .ok();
    }

    pub fn set<D>(&self, key: CacheKey, value: i32, duration: D) -> (i32, Duration)
    where
        D: Into<Duration>,
    {
        let res = (value, duration.into());
        self.cache_mut()
            .map(|mut t| {
                let res_clone = res.clone();
                let t = &mut *t;

                match key {
                    CacheKey::Year => {
                        t.year = Option::from(res_clone);
                    }
                    CacheKey::Month => {
                        t.month = Option::from(res_clone);
                    }
                    CacheKey::Day => {
                        t.day = Option::from(res_clone);
                    }
                    CacheKey::Hour => {
                        t.hour = Option::from(res_clone);
                    }
                    CacheKey::Minute => {
                        t.minute = Option::from(res_clone);
                    }
                    CacheKey::Second => {
                        t.second = Option::from(res_clone);
                    }
                    CacheKey::Millisecond => {
                        t.millisecond = Option::from(res_clone);
                    }
                    CacheKey::Microseconds => {
                        t.microseconds = Option::from(res_clone);
                    }
                    CacheKey::Nanosecond => {
                        t.nanosecond = Option::from(res_clone);
                    }
                    CacheKey::DayOfWeek => {
                        t.day_of_week = Option::from(res_clone);
                    }
                    CacheKey::Week => {
                        t.week = Option::from(res_clone);
                    }
                    CacheKey::DayOfYear => {
                        t.day_of_year = Option::from(res_clone);
                    }
                    CacheKey::DayOfMonth => {
                        t.day_of_month = Option::from(res_clone);
                    }
                }
            })
            .ok();
        res
    }

    pub fn get(&self, key: CacheKey) -> Option<(i32, Duration)> {
        self.cache()
            .map(|t| match key {
                CacheKey::Year => t.year.clone(),
                CacheKey::Month => t.month.clone(),
                CacheKey::Day => t.day.clone(),
                CacheKey::Hour => t.hour.clone(),
                CacheKey::Minute => t.minute.clone(),
                CacheKey::Second => t.second.clone(),
                CacheKey::Millisecond => t.millisecond.clone(),
                CacheKey::Microseconds => t.microseconds.clone(),
                CacheKey::Nanosecond => t.nanosecond.clone(),
                CacheKey::DayOfWeek => t.day_of_week.clone(),
                CacheKey::Week => t.week.clone(),
                CacheKey::DayOfYear => t.day_of_year.clone(),
                CacheKey::DayOfMonth => t.day_of_month.clone(),
            })
            .unwrap_or(None)
    }

    fn cache(&self) -> Result<RwLockReadGuard<'_, Cache>, DynTracerError> {
        Ok(self.0.read())
    }

    fn cache_mut(&self) -> Result<RwLockWriteGuard<'_, Cache>, DynTracerError> {
        Ok(self.0.write())
    }
}

impl Clone for CacheWrapper {
    fn clone(&self) -> Self {
        self.cache()
            .map(|t| Self(RwLock::new(t.clone())))
            .unwrap_or_default()
    }
}

pub enum CacheKey {
    Year,
    Month,
    #[allow(unused)]
    Day,
    Hour,
    Minute,
    Second,
    Millisecond,
    Microseconds,
    #[allow(unused)]
    Nanosecond,
    #[allow(unused)]
    DayOfWeek,
    #[allow(unused)]
    Week,
    #[allow(unused)]
    DayOfYear,
    DayOfMonth,
}

#[derive(Debug, Clone)]
pub struct Cache {
    pub(super) year: Option<(i32, Duration)>,
    pub(super) month: Option<(i32, Duration)>,
    pub(super) day: Option<(i32, Duration)>,
    pub(super) hour: Option<(i32, Duration)>,
    pub(super) minute: Option<(i32, Duration)>,
    pub(super) second: Option<(i32, Duration)>,
    pub(super) millisecond: Option<(i32, Duration)>,
    pub(super) microseconds: Option<(i32, Duration)>,
    pub(super) nanosecond: Option<(i32, Duration)>,
    pub(super) day_of_week: Option<(i32, Duration)>,
    pub(super) week: Option<(i32, Duration)>,
    pub(super) day_of_year: Option<(i32, Duration)>,
    pub(super) day_of_month: Option<(i32, Duration)>,
}

impl Default for Cache {
    fn default() -> Self {
        Self {
            year: None,
            month: None,
            day: None,
            hour: None,
            minute: None,
            second: None,
            millisecond: None,
            microseconds: None,
            nanosecond: None,
            day_of_week: None,
            week: None,
            day_of_year: None,
            day_of_month: None,
        }
    }
}
