use std::time::{SystemTime, UNIX_EPOCH};

pub struct ElapsedTime {
    pub start_time: Option<u64>,
    pub end_time: Option<u64>
}

impl Default for ElapsedTime {
    fn default() -> Self {
        Self {
            start_time: Some(get_current_time()),
            end_time: None
        }
    }
}
 
impl ElapsedTime {
    pub fn time_taken(&self) -> Option<u64> {
        match (self.start_time, self.end_time) {
            (None, _) => {
                None
            },
            (Some(_), None) => {
                None
            },
            (Some(start_time), Some(end_time)) => {
                Some(end_time - start_time)
            }
        }
    }

    pub fn has_elapsed(&self) -> bool {
        self.start_time.is_some() && self.end_time.is_some()
    }

    pub fn start(&mut self) {
        self.start_time = Some(get_current_time())
    }

    pub fn end(&mut self) {
        self.end_time = Some(get_current_time())
    }

}

pub struct WordTypingTime {
    pub word: String,
    pub time: ElapsedTime
}

impl WordTypingTime {
    pub fn for_word(word: String) -> Self {
        Self {
            word,
            time: Default::default()
        }
    }
}

fn get_current_time() -> u64 {
    let now = SystemTime::now();
    let now_from_epoch = now
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let in_ms = now_from_epoch.as_secs() * 1000 +
        now_from_epoch.subsec_nanos() as u64 / 1_000_000;

    in_ms
}
