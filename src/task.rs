#[derive(Serialise)]
struct Task {
    created_at: Date,
    running: bool,
    description: String,
    estimated_time: u64,
    sprints: Vec<Spring>,
}

#[derive(Serialise)]
struct Sprint {
    start_date: Date,
    end_date: Date,
}