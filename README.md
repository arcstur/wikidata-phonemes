# Wikidata Phonemes

Wikidata phoneme web editor suitable for mobile phones.

This is used in the [Wikidata IOLab activity](https://en.wikiversity.org/wiki/Wikidata_IOLab).

## Build

Build and deploy using the `docker-compose.yml` file:

```bash
docker-compose up --build
```

## Environment variables

* `OAUTH_CLIENT_ID` and `OAUTH_CLIENT_SECRET` are the Wikimedia OAuth consumer keys of your registered application.
* `RUST_LOG` defines the log level, we suggest: `RUST_LOG=info,tower_http=debug,wikidata_phonemes=debug,axum_login=warn,tower_sessions=warn`

### Development

* `DATABASE_URL` is the SQLite connection url and should point to the database file. The application uses the database file at `./phonemes.db`, so this should be `sqlite:phonemes.db`.
