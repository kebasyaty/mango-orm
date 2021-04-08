use mango_orm::*;
//use mongodb::bson::doc;

mod mango_models;
mod settings;

// Migration Service `Mango`.
fn mango_migration() -> Result<(), Box<dyn std::error::Error>> {
    // Caching MongoDB clients.
    {
        let mut client_store = MONGODB_CLIENT_STORE.write()?;
        client_store.insert(
            "default".to_string(),
            mongodb::sync::Client::with_uri_str("mongodb://localhost:27017")?,
        );
        client_store.insert(
            "default_2".to_string(),
            mongodb::sync::Client::with_uri_str("mongodb://localhost:27017")?,
        );
    }
    // Monitor initialization.
    let monitor = Monitor {
        project_name: settings::PROJECT_NAME,
        unique_project_key: settings::UNIQUE_PROJECT_KEY,
        // Register models.
        // <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
        models: vec![
            mango_models::Dynamic::meta()?,
            mango_models::User::meta()?,
            mango_models::UserProfile::meta()?,
        ],
    };
    monitor.migrat();

    // Add metadata and widgects map to cache.
    // Hint: Optional. It is required to add to work with the admin panel.
    // Admin panel: https://github.com/kebasyaty/mango-panel
    // <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
    /*
    mango_models::Dynamic::to_cache()?;
    mango_models::User::to_cache()?;
    mango_models::UserProfile::to_cache()?;
    */
    //
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Run migration.
    mango_migration()?;

    // Test dynamic widgets.
    // *********************************************************************************************

    /*
        println!(
        "{:?}",
        mango_models::Dynamic::db_update_dyn_widgets(
            r#"{
                "select_text_dyn":[["volvo","Volvo"],["saab","Saab"],["mercedes","Mercedes"],["audi","Audi"]],
                "select_text_mult_dyn":[["volvo","Volvo"],["saab","Saab"],["mercedes","Mercedes"],["audi","Audi"]],
                "select_i32_dyn":[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]],
                "select_i32_mult_dyn":[["-1","Volvo"],["-2","Saab"],["-3","Mercedes"],["-4","Audi"]],
                "select_u32_dyn":[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]],
                "select_u32_mult_dyn":[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]],
                "select_i64_dyn":[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]],
                "select_i64_mult_dyn":[["-1","Volvo"],["-2","Saab"],["-3","Mercedes"],["-4","Audi"]],
                "select_f64_dyn":[["1.1","Volvo"],["2.2","Saab"],["3.3","Mercedes"],["4.4","Audi"]],
                "select_f64_mult_dyn":[["-1.1","Volvo"],["-2.2","Saab"],["-3.3","Mercedes"],["-4.4","Audi"]]
            }"#
        )
        .is_ok()
    );
    */

    //println!("{:?}\n\n", mango_models::Dynamic::form_wig().unwrap());
    //println!("{}\n\n", mango_models::Dynamic::form_json().unwrap());
    //println!("{}\n\n", mango_models::Dynamic::form_html().unwrap());

    /*
    let mut dynamic = mango_models::Dynamic {
        select_text_dyn: Some("saab".to_string()),
        select_text_mult_dyn: Some(vec![
            "volvo".to_string(),
            "saab".to_string(),
            "audi".to_string(),
        ]),
        ..Default::default()
    };
    let result = dynamic.save(None, None, None)?;
    println!("Boolean: {}", result.bool());
    println!("Hash: {}", result.hash()?);
    println!("ID: {:?}", result.id()?);
    //println!("\n\nWidget map:\n{:?}", result.wig());
    //println!("\n\nJson:\n{}", result.json()?);
    //println!("\n\nHtml:\n{}", result.html());
    */

    // Test Model.
    // *********************************************************************************************
    // println!("{:?}\n\n", mango_models::UserProfile::form_wig().unwrap());
    // println!("{}\n\n", mango_models::UserProfile::form_json().unwrap());
    // println!("{}\n\n", mango_models::UserProfile::form_html().unwrap());

    //mango_models::UserProfile::find_one_and_delete(doc! {"username": "Rust"}, None)?;

    /*
    // Get boolean.
    println!(
        "Documents availability: {}\n\n",
        mango_models::UserProfile::find_one(Some(doc! {"username": "Rust"}), None)?.bool()
    );
    // Get raw document.
    println!(
        "Raw document:\n{:?}\n\n",
        mango_models::UserProfile::find_one(Some(doc! {"username": "Rust"}), None)?.raw_doc()
    );
    // Get prepared document.
    println!(
        "Prepared document:\n{:?}\n\n",
        mango_models::UserProfile::find_one(Some(doc! {"username": "Rust"}), None)?.doc()?
    );
    // Get json-line.
    println!(
        "Json-line:\n{}\n\n",
        mango_models::UserProfile::find_one(Some(doc! {"username": "Rust"}), None)?.json()?
    );
    // Get Model instance.
    println!(
        "Model instance:\n{:?}\n\n",
        mango_models::UserProfile::find_one(Some(doc! {"username": "Rust"}), None)?
            .model::<mango_models::UserProfile>()?
    );

    // Get boolean.
    println!(
        "Documents availability: {}\n\n",
        mango_models::UserProfile::find(Some(doc! {"username": "Rust"}), None)?.bool()?
    );
    // Get the number of documents.
    println!(
        "The number of documents: {}\n\n",
        mango_models::UserProfile::find(Some(doc! {"username": "Rust"}), None)?.count()?
    );
    // Get raw documents.
    println!(
        "Raw document:\n{:?}\n\n",
        mango_models::UserProfile::find(Some(doc! {"username": "Rust"}), None)?.raw_docs()?
    );
    // Get prepared document.
    println!(
        "Prepared document:\n{:?}\n\n",
        mango_models::UserProfile::find(Some(doc! {"username": "Rust"}), None)?.docs()?
    );
    // Get json-line.
    println!(
        "Json-line:\n{}\n\n",
        mango_models::UserProfile::find(Some(doc! {"username": "Rust"}), None)?.json()?
    );
    */

    /*
    println!(
        "\n\n{}\n\n",
        mango_models::UserProfile::form_json_for_admin()?
    );
    */

    // Test Model.
    let mut user = mango_models::UserProfile {
        username: Some("Rust".to_string()),
        email: Some("test_1_@test.test".to_string()),
        confirm_email: Some("test_1_@test.test".to_string()),
        password: Some("12345678".to_string()),
        confirm_password: Some("12345678".to_string()),
        date: Some("2020-12-19".to_string()),
        datetime: Some("2020-12-19T15:57".to_string()),
        num_i32: Some(-32),
        num_u32: Some(32),
        num_i64: Some(-64),
        num_f64: Some(-64.64),
        is_staff: Some(false),
        is_active: Some(true),
        select_text_mult: Some(vec!["1".to_string(), "2".to_string()]),
        select_i32_mult: Some(vec![1, 2]),
        select_u32_mult: Some(vec![1, 2]),
        select_i64_mult: Some(vec![1, 2]),
        select_f64_mult: Some(vec![1.0, 2.0]),
        ..Default::default() // or initialize the `hash` field - { hash: Some(String::new()) }
    };

    // println!("{}", user.json_for_admin()?);

    // Create doc.
    let result = user.save(None, None)?;
    println!("Boolean: {}", result.is_valid());
    println!("Hash: {}", result.hash()?);
    //println!("ID: {:?}", result.id()?);
    //println!("\n\nWidget map:\n{:?}", result.wig());
    //println!("\n\nJson:\n{}", result.json()?);
    //println!("\n\nHtml:\n{}", result.html());
    //println!("{}", result.json_for_admin()?);
    /*
    println!(
        "Verify password (false): {}",
        user.verify_password("123456789", None)?
    );
    println!(
        "Verify password (true): {}",
        user.verify_password("12345678", None)?
    );

    // Get Model instance.
    println!(
        "Model instance:\n{:?}\n\n",
        mango_models::UserProfile::find_one(Some(doc! {"username": "Rust"}), None)?
            .model::<mango_models::UserProfile>()?
    );
    */

    // Update doc.
    user.username = Some("Rust 2".to_string());
    //user.file = Some(r#"{"path":"","url":"","is_delete":true}"#.to_string());
    //user.image = Some(r#"{"path":"","url":"","is_delete":true}"#.to_string());
    let result = user.save(None, None)?;
    println!("\n\n\nBoolean: {}", result.is_valid());
    println!("Hash: {}", result.hash()?);
    //println!("ID: {:?}", result.id()?);
    //println!("\n\nWidget map:\n{:?}", result.wig());
    //println!("\n\nJson:\n{}", result.json()?);
    //println!("\n\nHtml:\n{}", result.html());
    //println!("{}", result.json_for_admin()?);
    /*
    println!(
        "Update password (false): {}",
        user.update_password("123456789", "123456789", None, None)?
    );
    println!(
        "Update password (true): {}",
        user.update_password("12345678", "123456789", None, None)?
    );
    println!(
        "Verify password (true): {}",
        user.verify_password("123456789", None)?
    );
    println!(
        "Verify password (false): {}",
        user.verify_password("12345678", None)?
    );
    */

    /*
    // Remove document.
    println!("Remove document: {:?}", user.delete(None)?);

    // Get count of documents.
    println!(
        "Estimated count of documents: {}",
        mango_models::UserProfile::estimated_document_count(None)?
    );
    */

    /*
    // Test Form.
    // *********************************************************************************************
    // println!("{:?}\n\n", mango_models::UserForm::form_wig().unwrap());
    // println!("{}\n\n", mango_models::UserForm::form_json().unwrap());
    // println!("{}\n\n", mango_models::UserForm::form_html().unwrap());

    println!("Form name: {}\n", mango_models::UserForm::form_name());
    println!(
        "Fields name:\n{:?}\n",
        mango_models::UserForm::fields_name()?
    );

    // Test Form.
    let user_form = mango_models::UserForm {
        username: Some("Rust".to_string()),
        email: Some("test_1_@test.test".to_string()),
        confirm_email: Some("test_1_@test.test".to_string()),
        password: Some("12345678".to_string()),
        confirm_password: Some("12345678".to_string()),
    };

    let result = user_form.check()?;

    println!("Boolean: {}\n\n", result.bool());
    //println!("Widget map:\n{:?}\n\n", result.wig());
    //println!("Json:\n{}\n\n", result.json()?);
    //println!("Html:\n{}\n", result.html());
    println!(
        "Form instance:\n{:?}\n",
        result.form::<mango_models::UserForm>()
    );
    */

    Ok(())
}
