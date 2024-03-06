pub fn update_counter() -> Result<CounterData, Error> {
    let year = Local::now().naive_local().year();

    let mut file = OpenOptions::new()
        .read(true)
        .create(true)
        .open("count.json")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut data: CounterData = CounterData { year, count: 0 };

    if !contents.is_empty() {
        data = serde_json::from_str(&contents)?;
    }

    data.count += 1;

    let mut file = File::create("count.json")?;
    file.write_all(serde_json::to_string(&data)?.as_bytes())?;

    Ok(data)
}
