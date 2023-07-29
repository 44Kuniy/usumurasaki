# About

- This repository aims to consolidate my knowledge and write program code."
- The contents consist of code written as a hobby.
- There is also the purpose of making it useful for job hunting (though it's just an extra).

# Contents

- This is website-project aiming to redirect users to Amazon product page with my affiliate tag.
- The displayed products are limited to those featured on the Youtube channel.

# How

1. Enable the import of data from a CSV file.
2. Generate metadata so users can search for products easily.
3. Redirect users to the Amazon product page with my affiliate tag.

# Directory Structure

## Backend

Written mainly in Rust language.
**`w-`prefix means cargo workspace. mainly for purpose of code search.**

- [w-config](./backend/w-config/)
  - initialize config from env files.
- [w-db](./backend/w-db/)
  - define functions to load data and migrations.
- [w-external-api](./backend/w-external-api)
  - handle external Api service.
- [w-importer](./backend/w-importer/)
  - handle to import data from csv.
- [w-models](./backend/w-models/)
  - define entities.
  - [er-diagram](./er-diagram.md)
- [w-server](./backend/w-server/)
  - web server.
- [w-usecase](./backend/w-usecase/)
  - define usecases that calls Repo functions from `w-db`

## Frontend

Written in React

- izee
  - for public users
