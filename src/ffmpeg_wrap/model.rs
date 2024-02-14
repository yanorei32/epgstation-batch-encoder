pub struct FfmpegProgress {
    pub(in crate::ffmpeg_wrap) current_secs: u64,
    pub(in crate::ffmpeg_wrap) total_secs: u64,
}

impl FfmpegProgress {
    pub fn current_secs(&self) -> u64 {
        self.current_secs
    }

    pub fn total_secs(&self) -> u64 {
        self.total_secs
    }
}
