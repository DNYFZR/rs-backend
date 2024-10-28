# Rusty Backend

Side project which runs searches of the Github API and store the results in a NoSQL styled SQLite database.

## API Module

This extracts the top 10 starred repos from the Github API, using the users input as the search term for the API.

## Database Module

This handles updates of the sqlite database `app.db`

The DB contains a single table with two columns :

- name : text primary key
- data : JSON
