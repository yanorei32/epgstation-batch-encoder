pub struct FfmpegProgress {
    current_secs: u64,
    total_secs: u64,
}

impl FfmpegProgress {
    pub(in crate::ffmpeg_wrap) fn new(current_secs: u64, total_secs: u64) -> Self {
        Self {
            current_secs,
            total_secs,
        }
    }

    pub fn current_secs(&self) -> u64 {
        self.current_secs
    }

    pub fn total_secs(&self) -> u64 {
        self.total_secs
    }
}
