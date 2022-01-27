struct Task {
    start_date: Date,
    end_date: Option<Date>,
    active: bool,
    description: String,
    estimated_time: u64,
}