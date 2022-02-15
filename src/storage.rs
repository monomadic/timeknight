use crate::state::*;

pub(crate) fn load_state() -> Result<App, crate::Error> {
    let mut path = dirs::home_dir().expect("could not find $HOME directory");
    path.push(".timeknight");
    path.push("active.ron");
    // if no state can be found, create default

    if path.exists() {
        let data = std::fs::read_to_string(path)?;
        let tasks = ron::from_str(&data)?;
        Ok(App {
            tasks,
            ..Default::default()
        })
    } else {
        Ok(App::default())
    }
}

pub(crate) fn save_state(app: &App) -> Result<(), crate::Error> {
    let mut path = dirs::home_dir().expect("could not find $HOME directory");
    path.push(".timeknight");

    // create `$HOME/.timeknight` dir if missing
    std::fs::create_dir_all(&path)?;

    path.push("active.ron");
    std::fs::write(path, ron::ser::to_string_pretty(&app.tasks, ron::ser::PrettyConfig::new())?)?;
    Ok(())
}

pub(crate) fn save_completed_task<CT: Into<CompletedTask>>(task: CT) -> Result<(), crate::Error> {
    let mut path = dirs::home_dir().expect("could not find $HOME directory");
    path.push(".timeknight");
    path.push("completed_quests");

    // create `$HOME/.timeknight/completed_quests dir if missing
    std::fs::create_dir_all(&path)?;

    let task = task.into();

    path.push(&task.description.clone());
    std::fs::write(path, ron::ser::to_string_pretty(&task, ron::ser::PrettyConfig::new())?)?;
    Ok(())

}
