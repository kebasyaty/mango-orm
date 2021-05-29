![Logo](https://github.com/kebasyaty/mango-orm/raw/master/metamorphose/images/logo.svg)

# metamorphose

### Macros collection for converting Structure to Model, for a [mango-orm](https://github.com/kebasyaty/mango-orm "mango-orm") project.

![crates.io](https://img.shields.io/crates/v/metamorphose)
![crates.io](https://img.shields.io/crates/d/metamorphose)
![crates.io](https://img.shields.io/crates/l/metamorphose)

## Macros
#### Model
> Macro for converting Structure to mango-orm Model.
> The model can access the database.
> The model can create, update, and delete documents in collections.

#### Form
> Macro for converting Structure to mango-orm Form.
> The form does not have access to the database.
> Form are needed where it makes no sense to use a model -
> To create a search form, to recover a password, to combine models, etc.

## Requirements
- quote
- regex
- serde_json
- syn
- serde

## License
#### This project is licensed under the [MIT](https://github.com/kebasyaty/mango-orm/blob/master/LICENSE-MIT "MIT") and [Apache Version 2.0](https://github.com/kebasyaty/mango-orm/blob/master/LICENSE-APACHE "Apache Version 2.0")
