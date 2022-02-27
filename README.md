# Overdue! (Database Server Backend API) <a name="top"></a>

<p align="center"><img alt="Overdue! Logo" width="420px" src="./images/overdue-logo.png"></p>

![POWERED BY: ISTD SUTDENTS](https://img.shields.io/badge/powered%20by-istd%20SUTDents-73af44?style=for-the-badge&labelColor=d7ef32) [![Codecov](https://img.shields.io/codecov/c/gh/jamestiotio/overdue_backend?logo=codecov&style=for-the-badge)](https://codecov.io/gh/jamestiotio/overdue_backend) [![Dependency Status](https://deps.rs/repo/github/jamestiotio/overdue_backend/status.svg)](https://deps.rs/repo/github/jamestiotio/overdue_backend)

[![Build, Run Tests & Deploy](https://img.shields.io/github/workflow/status/jamestiotio/overdue_backend/Build%2C%20Run%20Tests%20%26%20Deploy?label=Build%2C%20Run%20Tests%20%26%20Deploy&logo=github&style=for-the-badge)](https://github.com/jamestiotio/overdue_backend/actions/workflows/main.yaml) [![Security Audit](https://img.shields.io/github/workflow/status/jamestiotio/overdue_backend/Security%20Audit?label=Security%20Audit&logo=github&style=for-the-badge)](https://github.com/jamestiotio/overdue_backend/actions/workflows/audit.yaml) [![Daily Security Audit](https://img.shields.io/github/workflow/status/jamestiotio/overdue_backend/Daily%20Security%20Audit?label=Daily%20Security%20Audit&logo=github&style=for-the-badge)](https://github.com/jamestiotio/overdue_backend/actions/workflows/daily-audit.yaml)

Backend Database Server Code of `Overdue!` (SUTD Fablab ISTD Game) for Open House 2021.

Game obviously inspired by [Overcooked!](https://www.team17.com/games/overcooked/).

Frontend code repository for the [Phaser](https://phaser.io/)-based public-facing web game client is accessible [here](https://github.com/nicolefranc/sutd-fablab-game).

While they are still up, the front-facing frontend game client can be accessed [here](https://sutd-fablab-game.netlify.app/) (the public-facing one can be seen [here](https://overdue.sutd.edu.sg/)) and the backend game server can be accessed [here](https://sutdoverdue.dev/). Our SUTD Open House subpage/category is located [here](https://openhouse.sutd.edu.sg/overdue/).

For players who contributed to the game's final goal of total materials collected, if all 3 project prototypes (1 for each difficulty stage/level) are unlocked by the end of SUTD Open House 2021, they will earn this badge:

<p align="center"><a href="https://au.badgr.com/public/badges/iQfJV6FcSP2cMQlfVyXBug"><img alt="SUTD Overdue! Game Contributor" width="100px" src="https://media.au.badgr.com/uploads/badges/6ce4a23d-ff9f-4d57-bc92-c24724cf569d.svg"></a></p>

Additionally, the top 3 players on the leaderboard of each difficulty will walk away with SGD$20 GrabFood vouchers each!

> The dense ranking algorithm is utilized to determine the rank of each score entry (instead of the standard competition ranking algorithm). The winners will be initially filtered and identified by email and timestamp entries in the database (as well as some rationality, logic and common sense regarding the physically possible maximum achievable total score). If an entry is deemed to be invalid (non-existing email, spam-requests cheating based on timestamp, ridiculously high score, etc.), the next entry in line will be considered. Further (identity document-based) verification by SUTD's administrative offices will also then be conducted behind the scenes to ensure that they are all **unique** individuals/persons (instead of just different disposable emails pointing to/owned by the same person). This ensures that no single individual "hogs"/"claims" all of the available awards/rewards. This also implies that the "top 3" entries on the leaderboard of each difficulty might not actually/accurately represent the "only" individuals who will win the vouchers. Therefore, do try your best to get a high score even if your entry is not listed/displayed on the in-game leaderboard since you might still have a chance to be the "top 3"!



## Table of Contents <a name="toc"></a>

- [Overdue Backend](#top)
  - [Table of Contents](#toc)
  - [Justification of Design Decisions](#design)
  - [Setup Instructions](#setup)
  - [API Endpoints](#api)
  - [Future Development Notes](#future-dev)
  - [Acknowledgements](#acknowledgements)



## Justification of Design Decisions <a name="design"></a>

[back to top](#top)

> Server performance and data safety/security are the *sine qua non* for this project. To be very honest, this is a ridiculously super over-engineered no-nonsense implementation for *...ahem... [serious business purposes](https://github.com/EnterpriseQualityCoding/FizzBuzzEnterpriseEdition)* (although due to time constraint, this project is not really to that level of no-nonsense... yet?). But I consider it a good thing. Welp. 🤷 Fight me. ᕕ( ᐛ )ᕗ

The reason we utilized [Rust](https://www.rust-lang.org/) is due to its performance, reliability, memory and thread safety, easier concurrency and zero-cost abstractions. It also addresses and solves a lot of pain points present in many other programming languages (with a couple of advantages over/compared to languages such as JavaScript, C/C++ and Go). Its strong and static type system allows us to conduct some initial input validation even before any data is being passed on to inner/deeper functions, ensuring better security of our backend server application. Rust's performance in terms of speed is even comparable to `C++ g++` and `C gcc` (source [here](https://benchmarksgame-team.pages.debian.net/benchmarksgame/which-programs-are-fastest.html)). The reason we utilized [`actix-web`](https://actix.rs/) is because it is production-ready, battle-tested, powerful, pragmatic, type-safe and extremely fast/performant (check out its performance on [TechEmpower Framework Benchmarks](https://www.techempower.com/benchmarks/), keeping in mind the usual caveats of such benchmarks). It is also the top-performing web framework written in Rust, thereby allowing us to achieve high performance and high security. The reason we utilized [`tokio-postgres`](https://github.com/sfackler/rust-postgres) is because it utilizes the same Tokio runtime used by Actix. As such, this allows us to achieve this synergy of mutual reinforcement and compatibility for the sake of data manipulation, data formatting and data types. It also has awesome community support, which is greatly helpful since there are many helper crates that were able to improve our productivity rate for the sake of delivering the MVP (minimum viable product) of this backend server as we were able to focus better and more on creating the main business logic of this application instead of having to deal with other more menial tasks such as struct data type conversions and complicated manual input validation.

The validation technique itself is not impenetrable/unbreakable/unspoofable. Some form of end-to-end encryption or advanced cookie-based authentication could be used in the future to guarantee that nobody could inspect the content of the packets, but there is no avoiding the fact that it is entirely possible for malicious users to pose/camouflage as regular players and send legal, even though physically impossible to achieve (or probable with very low probability since it requires a whole lot of luck), JSON payloads. We could obfuscate the payload but it just distances away the possibility of cheating and hacking with more effort, instead of completely preventing them from happening in the first place. We have separated the frontend and backend elements for the sake of preventing arbitrary code execution where malicious scripts could be easily run on the same server (which is much harder to prevent considering that any credentials put on the frontend game client side will be exposed to all users, no matter how uglified/obfuscated it might be). Possible foolproof authentication methods would be to use JWT tokens or cookies as session identifiers (with unique values of user agents, timestamps, user client ID hashes/keys, etc.) in conjunction with environment variables (since JavaScript is sandboxed from the OS). However, this is also not foolproof since someone can always pretend to be the frontend server by simply copying the cookie/token value. The issue that we face here is much more complex and complicated compared to the usual CIA properties of the conventional network security problem (confidentiality, integrity, authentication, access and availability) since we do not implement any levels of authentication/login system on the frontend side. We are not attempting to prevent other users from pretending to be a specific user (this issue is resolved by using TLS and HTTPS), but instead we are attempting to prevent the user with 100% access to the frontend server code from pretending to be the frontend server itself! It's an entirely different and separate issue. As such, we are actually especially prone to replay attacks. We have implemented validation techniques to the point whereby only legal payloads are accepted. Even then, there are payloads which are still considered to be valid (since they are *technically* possible within the limits of our application), and yet it might seem obvious to the human eyes that such payloads are definitely considered to be the result of cheating instead of proper play of the game. We could implement behaviour tracking on the game client side to identify which payloads have a high degree/probability of being the result of cheating, but even those scripts could definitely be circumvented somehow (since we do not have full control over the players' devices anyway to install any anti-cheating tools, which would be ethically/morally questionable to some elements of invasion of privacy). The point of the validation is to provide enough distance to deter most people from attempting to cheat. However, if anyone is willing and able enough to put in the necessary time and effort to circumvent the validation, it is definitely possible. All games are cheatable and hackable. However, if the validation is implemented in such a way that someone who is willing to cheat will need to put in the same (or perhaps even more) required amount of effort and time as just playing the game legally, I consider that validation as a success. Figuring out how to hack/cheat the game will be the game itself for said hacker/cheater. Besides, I am honored if someone attempted to cheat/hack at this small game of ours.

Developing this server code prototype was quite enjoyable, even though the timeline was pretty intensive. Maybe I will become a Rustacean one day. 😌



## Setup Instructions <a name="setup"></a>

[back to top](#top)

> It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev) to verify the trustworthiness of each of your dependencies, including this one.

1. Download and install Rust with its toolchain (including `rustc`, `cargo` and `rustup`) from [here](https://www.rust-lang.org/tools/install). After that, optionally run these commands to use Clippy:

    ```cmd
    > rustup self update
    > rustup default stable
    > rustup component add clippy
    ```

2. The code can be formatted by using the nightly cargo version and rustfmt:

    ```cmd
    > rustup self update
    > rustup default nightly
    > rustup component add rustfmt --toolchain nightly
    > cargo +nightly fmt
    ```

3. If you are on Ubuntu/Linux or MacOS, ensure that you have the usual C/C++ toolchain, OpenSSL and PostgreSQL installed. Follow the instructions [here](https://www.postgresql.org/download/) as per normal, run these commands:

    ```console
    $ sudo apt install build-essential checkinstall zlib1g-dev openssl libssl-dev postgresql-13 postgresql-client-13 libpq-dev pgadmin4
    $ sudo systemctl enable postgresql
    $ sudo update-rc.d postgresql enable
    ```

    If you are on Windows, ensure that you have installed `Visual Studio with the Desktop C++ workload and the Windows SDK for Desktop C++`, [`vcpkg`](https://github.com/microsoft/vcpkg), `OpenSSL`, `Cygwin` and `PostgreSQL`. To install PostgreSQL, download the installer from [here](https://www.postgresql.org/download/windows/) and install it normally. To install Visual Studio, go to [here](https://visualstudio.microsoft.com/downloads/) to download the installer and follow the instructions [here](https://docs.microsoft.com/en-us/visualstudio/install/install-visual-studio). After that, select a convenient (and preferably short with no whitespaces, as advised by `vcpkg`'s main `README.md` file) directory path of your choice for your `vcpkg` installation. Then, set up your `vcpkg` and `OpenSSL` properly:

    ```cmd
    > git clone https://github.com/microsoft/vcpkg
    > cd vcpkg\
    > .\bootstrap-vcpkg.bat -disableMetrics
    > vcpkg integrate install
    > vcpkg install openssl:x64-windows openssl:x86-windows
    > vcpkg --triplet x64-windows-static-md install openssl
    ```

    > Portable versions of `cmake`, `7zip`, `nuget` and `powershell-core` will be automatically downloaded and extracted by `vcpkg`.

4. Ensure that only access by `localhost` is allowed for PostgreSQL. The following `pg_hba.conf` configuration should be sufficient:

    ```
    # TYPE  DATABASE        USER            ADDRESS                 METHOD
    local   all             all                                     trust
    host    overdue         overdue       192.168.1.1/32            md5
    host    overdue         overdue       172.17.0.0/16             md5
    host    all             all           0.0.0.0/0                 reject
    ```

5. Alternatively, you can set up the PostgreSQL database service by using Docker so as to isolate the container's network from the host's network. Install [Docker Engine](https://docs.docker.com/engine/install/), [Docker Compose](https://docs.docker.com/compose/install/) and [PostgreSQL](https://www.postgresql.org/download/) first. Then, run this command:

    ```cmd
    > docker-compose up -d
    ```

    Keep in mind that the system's `postgresql` service should be disabled since the `postgres` Docker service might be in conflict and become unable and fail to bind to the `0.0.0.0:5432` address/port (since the address is already in use). This can be done by running this command:

    ```console
    $ sudo systemctl stop postgresql.service
    ```

6. To set up the database, run this command:

    ```cmd
    > psql -h 127.0.0.1 -p 5432 -U overdue overdue < database.sql
    ```

7. Enable security features for the backend server code. There are two things: firstly, follow the instructions [here](https://letsencrypt.org/getting-started/) and [here](https://certbot.eff.org/instructions) to generate the SSL certificate for HTTPS support (at least TLS v1.2) using Certbot with Let's Encrypt as its Certificate Authority and put the certificate into the appropriate folder directory (this would require a domain name for the backend server so go and obtain one from your nearest domain name registrar). For our case, we utilize Namecheap as our domain name registrar (remember to enable WhoisGuard, PremiumDNS and DNSSEC). Add the corresponding DNS Host Records:

    | Type | Host | Value | TTL |
    | --- | --- | --- | --- |
    | A Record | `@` | `<DigitalOcean Droplet's IPv4 Address>` | 60 min |
    | A Record | `www` | `<DigitalOcean Droplet's IPv4 Address>` | 60 min |
    | CAA Record | `@` | `0 issue letsencrypt.org` | 60 min |

    Secondly, get a domain name for the game client as well for the purposes of the CORS policy. Preferably, the domains should have TLD-level HSTS enabled (i.e., included in the HSTS preload secure namespace list, such as `.APP` or `.DEV` domains) so as to force secure HTTPS connections through browsers by default, which actually does not affect the Let's Encrypt ACME Challenge verification process (HTTP connections would not be automatically redirected to HTTPS since [this PR](https://github.com/petertrotman/actix-web-middleware-redirect-https/pull/4) is not merged yet). Regarding prevention of XSS attacks, the `Content-Security-Policy` header should be automatically set by `actix-web` and the TLS implementation for this back-end server. The front-end game client would include the corresponding appropriate `Content-Security-Policy` header as well. By default, the backend server code will be rate-limited and will possess enough necessary input validation and verification (any malformed, incompatible or invalid input data will be automatically rejected). Use `iptables` to limit the number of SSH connection attempts. Install `fail2ban` to further protect the SSH port 22. Use the SSH key pairs provided by DigitalOcean. Use port knocking. Do not allow root logins. Restrict access to a named group. Do not use shared logins. Do not allow direct access below the presentation tier. Optionally, we can also set up a dedicated Droplet host acting as a SSH jump server box. For additional security, a load balancer could also be deployed to mitigate DDoS attacks. Also remember to enable Droplet Backups so that we have data backup. These are the settings for the DigitalOcean Cloud Firewall:

    - Inbound Rules:
      | Type | Protocol | Port Range | Sources |
      | --- | --- | --- | --- |
      | SSH | TCP | 22 | `All IPv4` `All IPv6` |
      | HTTP | TCP | 80 | `All IPv4` `All IPv6` |
      | HTTPS | TCP | 443 | `All IPv4` `All IPv6` |
      | Custom (for Git) | TCP | 9418 | `All IPv4` `All IPv6` |

    - Outbound Rules:
      | Type | Protocol | Port Range | Destinations |
      | --- | --- | --- | --- |
      | ICMP | ICMP |  | `All IPv4` `All IPv6` |
      | All TCP | TCP | All ports | `All IPv4` `All IPv6` |
      | All UDP | UDP | All ports | `All IPv4` `All IPv6` |

    For testing purposes, you can generate your own self-signed certificate-key pair of `cert.pem` and `privkey.pem` by running these commands (on Ubuntu/Linux or a UNIX-based system equipped with OpenSSL):

    ```console
    $ touch ~/.rnd
    $ dd if=/dev/urandom of=~/.rnd bs=256 count=1
    $ sudo su -c 'openssl req -x509 -newkey rsa:4096 -nodes -sha512 -keyout tls/privkey.pem -out tls/cert.pem -days 365 -subj "/CN=localhost" -extensions EXT -config <(printf "[dn]\nCN=localhost\n[req]\ndistinguished_name = dn\n[EXT]\nsubjectAltName=DNS:localhost\nkeyUsage=digitalSignature\nextendedKeyUsage=serverAuth")'
    ```

    For the Postgres database server, since we cannot generate a TLS certificate for `localhost` using Let's Encrypt (more information [here](https://letsencrypt.org/docs/certificates-for-localhost/)), we can use either another self-signed certificate or use [`minica`](https://github.com/jsha/minica) (re-using the aforegenerated SSL certificate for our public Internet-facing backend server domain is definitely out of the question since the local machine's DNS configuration would need to be configured to have the same domain name as the server until it is *almost out of whack*).

8. Optionally, follow [this tutorial](https://www.digitalocean.com/community/tutorials/how-to-securely-manage-secrets-with-hashicorp-vault-on-ubuntu-16-04) to set up HashiCorp Vault for the DigitalOcean VPS for the purpose of storing and accessing/reading environment variables and credentials securely. Remember to use TLS certificates, enable Consul encryption and enable ACLs to make it production-ready. Alternatively, set the appropriate environment variables and credentials for the backend app server (such as the TLS certificates and the PostgreSQL database credentials).

9. For linting and testing, run these commands (use `cargo-tarpaulin` to get the code lines test coverage percentage):

    ```cmd
    > cargo clippy
    > cargo test --all-features -- --test-threads=1
    > cargo tarpaulin --verbose --release --all-features --all-targets --tests --workspace --out Xml -- --test-threads=1
    ```

10. For development, run these commands:

    ```cmd
    > cd overdue_backend\
    > cargo install cargo-watch
    > cargo build
    > cargo watch -x run
    ```

    NOTE: You might need to change the `SERVER__PORT` environment variable in the `.env` file to `8443` instead of `443` if you encounter some permission denied issue on your local machine during development.

11. For release, you can install the binary by running `cargo install --bin overdue_backend --path .` or by running these commands (by default, the executable is placed in the `./target/release` folder):

    ```cmd
    > cd overdue_backend\
    > cargo install cargo-deb
    > cargo build --bin overdue_backend --release
    > cargo run --release
    ```

    We are using GitHub Actions for Continuous Integration and Continuous Delivery. Alternatively, you can follow this [tutorial](https://www.digitalocean.com/community/tutorials/how-to-install-and-configure-drone-on-ubuntu-20-04) to run tests using Drone CI. We are using SSH as our method of deployment since the alternative would be by using the [`doctl`](https://github.com/digitalocean/action-doctl) CLI, which is sort of more dangerous/risky in terms of security/safety since instead of potentially "exposing" the SSH key to a single Droplet instance, we might "expose" a whole DigitalOcean PAT API key with read and write permissions in my DigitalOcean account (with the tradeoff of being slightly less robust due to the hardcoded absolute paths to the executable binaries). As such, please be reminded to specify the specific SSH `id_rsa` private keyfile with no passphrase (by using `-i ~/.ssh/id_rsa`), specify the specific SSH `known_hosts` file (by using `-o UserKnownHostsFile=~/.ssh/known_hosts` to avoid the warning of non-establishable ECDSA key fingerprint authenticity of the host) and configure the `$PATH` environment variables accordingly so as to be able to properly run any executable binaries since SSH is a non-interactive shell (perhaps by using absolute paths or by installing the needed executables using the official Ubuntu's package manager `apt`). To allow Git to checkout, clone, pull and merge this repository, we utilize a [read-only deploy key](https://github.blog/2015-06-16-read-only-deploy-keys/) installed on the target server machine (instructions specified [here](https://docs.github.com/en/developers/overview/managing-deploy-keys#deploy-keys)). Before deployment, ensure that the DigitalOcean Droplet has enough memory (RAM) since if not, it will run out of memory (OOM) and will fail to compile and hence deploy as the scheduler in the system/kernel will send a `SIGKILL` signal to the `rustc` compiler if it takes up too much memory. Simply re-running the workflow until it achieves a successful deployment should solve this issue.

12. As when the server is live during production, if the tables' properties need to be altered for some whatever reason, we can do so by running the `ALTER TABLE` SQL command (refer to the [documentation](https://www.postgresql.org/docs/current/sql-altertable.html) for more information).

    We can count the number of entries/games played and unique emails submitted to the leaderboard respectively by running this command:

    ```sql
    # SELECT COUNT(*) FROM leaderboard;
    # SELECT COUNT(DISTINCT email) FROM leaderboard;
    ```

    For the purposes of final statistics and to assist in distributing the virtual custom badges to the different players and contributors, we can get all the unique emails in the leaderboard and output them into a single CSV file by running this command (we follow [Badgr](https://badgr.com/)'s sample CSV template format from [here](https://docs.google.com/spreadsheets/d/1mvrTrtx-IllkLXHVLdZBZlybCtlvklGe5suiVUpLXPE)):

    ```console
    $ psql -h 127.0.0.1 -p 5432 -U overdue -d overdue -c 'COPY (SELECT DISTINCT email AS "Identifier" FROM leaderboard) TO STDOUT WITH CSV HEADER' > emails.csv
    ```

    An alternative command would be this (might either differ slightly in terms of formatting or produce an identical output for our very specific case):

    ```console
    $ psql -h 127.0.0.1 -p 5432 -U overdue -d overdue -c 'SELECT DISTINCT email AS "Identifier" FROM leaderboard' --csv > emails.csv
    ```

    For the purpose of giving out the GrabFood vouchers, we can select the top 3 unique emails for each difficulty by running this command:

    ```console
    $ for i in {0..2}; do (echo "SELECT MIN(subquery.id) AS id, subquery.email, subquery.difficulty, MIN(subquery.rank) AS rank FROM (SELECT id, email, difficulty, dense_rank() OVER (PARTITION BY difficulty ORDER BY score DESC) rank FROM leaderboard WHERE difficulty = $i) subquery GROUP BY subquery.email, subquery.difficulty ORDER BY rank ASC FETCH FIRST 3 ROWS ONLY;" | psql -h 127.0.0.1 -p 5432 -U overdue -d overdue --csv; echo "") >> vouchers.csv; done
    ```

    To dump the entire database onto CSV files to be exported and backed up onto other platforms or used on other Droplets or instances of Postgres or SQL databases, we can run these commands:

    ```console
    $ psql -h 127.0.0.1 -p 5432 -U overdue -d overdue -c 'SELECT * FROM leaderboard' --csv > leaderboard.csv
    $ psql -h 127.0.0.1 -p 5432 -U overdue -d overdue -c 'SELECT * FROM material' --csv > material.csv
    ```

    And finally, for cleanup, stop the running executable file/process (or background service, if we are using `systemd` and `systemctl`) and take down/shutdown the `postgres` Docker service by running these commands:

    ```console
    $ sudo systemctl stop overdue_backend.service
    $ sudo dpkg --purge overdue_backend
    $ docker-compose down
    $ sudo rm -rf overdue_backend/
    ```



## API Endpoints <a name="api"></a>

[back to top](#top)

There are 3 open endpoints:

- `/submit_score`: Submit the current score to the database. Use a POST request method (to `https://sutdoverdue.dev/submit_score`) with the request header of `Content-Type: application/json`. The request JSON data format is as follows:

  ```json
  {
    "name": "<some-name-sanitised-by-james>",
    "gender": "m|f|M|F",
    "email": "<whoo-some-secret-email-that-is-not-supposed-to-be-displayed-to-the-public>",
    "difficulty": "easy|normal|hard",
    "score": <some-integer>,
    "bonus": <some-integer>,
    "materials": [
      {
        "name": "<some-unique-item-name-string>",
        "quantity": <some-integer>
      },
      {
        "name": "<yet-another-unique-item-name-string>",
        "quantity": <some-integer>
      },
      ...
    ]
  }
  ```

  The `bonus` key is optional. If it is never specified, the default value is `0`.

  The JSON payload size limit is 1 KiB (which should be able to handle the maximum stretchable legal limit of each key's value, as well as a pretty decent length of the email key's value). Anything else larger than that will be rejected since it will be considered as a malicious spam payload (perhaps from a DDoS attempt or from a MitM-tampered payload).

  A successful response JSON data format is as follows:

  ```json
  {
    "name": "<some-name-sanitised-by-james>",
    "gender": "m|f",
    "difficulty": "easy|normal|hard",
    "score": <some-integer>,
    "rank": <some-integer>
  }
  ```

- `/get_leaderboard`: Get the first top `n` entries of the leaderboard of the specified difficulty level, where `n >= 0` and `n ∈ ℤ` (`n` is a non-negative integer), in terms of descending order of the score (JSON array is ordered). Use a GET request method, and add the optional `max_entries` and `difficulty` query parameters to the URL (they are unordered): `https://sutdoverdue.dev/get_leaderboard?max_entries=n&difficulty=easy|normal|hard`. The available options for the `difficulty` string/text query parameter are: `easy`, `normal`, and `hard`. If not specified, the default value is `10` for `max_entries` and `normal` for `difficulty`. The response JSON data format is as follows (already in descending order):

  ```json
  [
    {
      "name": "<some-sanitised-name-yay>",
      "gender": "m|f",
      "score": <some-integer>,
      "rank": <some-integer>
    },
    {
      "name": "<yet-another-sanitised-name-yay>",
      "gender": "m|f",
      "score": <some-integer>,
      "rank": <some-integer>
    },
    ...
  ]
  ```

- `/get_materials`: Get the total count of each material that has been submitted to the database. Just use a simple GET request method (to `https://sutdoverdue.dev/get_materials`), with no additional query parameters whatsoever. The response JSON data format is as follows:

  ```json
  [
    {
      "name": "<some-material-name>",
      "quantity": <some-integer>
    },
    {
      "name": "<yet-another-material-name>",
      "quantity": <some-integer>
    },
    ...
  ]
  ```

For the `/submit_score` and `/get_materials` endpoints, except for the `bonus` key entry of the `/submit_score` endpoint, all the other methods/entries/specifications/keys/parameters are REQUIRED (not optional) and any additional URL query parameters will be ignored (i.e., they will not affect the final result), otherwise a `400 Bad Request` error will be returned. For the `/get_leaderboard` endpoint, the `max_entries` and `difficulty` query parameters are optional and if those parameters are filled with wrong/incompatible/unserializable data that does not follow the aforementioned specified format, a `400 Bad Request` error will be returned. An HTTPS connection is mandatory/compulsory. Any non-HTTPS connections, any connections from any other domains (as indicated by the corresponding CORS policy) and any requests that does not comply within the stated specifications will be rejected/ignored. For all endpoints, the `Host` header must be specified with the correct value, otherwise a `404 Not Found` error will be returned. A `200 OK` HTTP response code should be received if everything is executed successfully. If there are any errors, the corresponding endpoint would return another error code (such as `400 Bad Request` or `500 Internal Server Error`), along with the corresponding error message content describing what error specifically has occurred (which might be useful for debugging purposes).

If a wrong endpoint resource is specified, a `404 Not Found` error will be returned. Else, if a wrong method is used, a `405 Method Not Allowed` error will be returned (due to the method guards being implemented). Else, if the rate limit is exceeded for a particular IP address, a `429 Too Many Requests` error will be returned. Otherwise, if an error is encountered, this will be the response JSON data format:

```json
{
  "code": <some-response-code>,
  "message": "<some-error-message>",
  "error": "<some-error-description>"
}
```

The error handler is set up to avoid unwanted panics and the error message is purposefully vague and not too specific so as to avoid prying eyes from figuring out and reverse-engineering the specific cause of error and forming a malicious payload that fits within the reasonable limits of our application. The validation techniques that have been implemented limit the crafting of malicious payloads to the level/point whereby the amount of effort required (as well as the level of "ability"/"power" gained/obtained from such an act) to reverse engineer the backend server code would be roughly similar to the amount of effort required to reverse engineer the scoring implementation on the game client side, which should be considered as reasonable enough for a decent application in production.



## Future Development Notes <a name="future-dev"></a>

[back to top](#top)

- For an extreme level of optimization, might want to consider using [Protocol Buffers](https://developers.google.com/protocol-buffers) instead of JSON so as to guarantee type-safety, prevent schema violations, provide simpler data accessors, have a smaller size of data being transferred around and enjoy faster serialization/deserialization not just from the server side, but also even from the client side. Consider using [`actix-protobuf`](https://github.com/actix/actix-extras/tree/master/actix-protobuf) when there is more community support for integration and compatibility between the Protobuf data format, `actix-web`, `tokio-postgres` and PostgreSQL as a whole. Possible to optimize even further to using [HDF5](https://www.hdfgroup.org/solutions/hdf5/) (use [this](https://github.com/aldanor/hdf5-rust)), [MessagePack](https://msgpack.org/), [CBOR](https://cbor.io/), [FlatBuffers](https://google.github.io/flatbuffers/) or even all the way to [Cap'n Proto](https://capnproto.org/) (use [this](https://github.com/capnproto/capnproto-rust)). Other less-known serializers might not have enough community or compatibility support for our real-life application setup (such as [Apache Thrift](https://thrift.apache.org/), [`bitsery`](https://github.com/fraillt/bitsery), [YAS](https://github.com/niXman/yas) or [`zpp`](https://github.com/eyalz800/serializer)). A hand-written serializer might be the best, but it will take too much time to write (unfeasible due to the given tight timeframe). Might want to also convert the frontend client component to using `yew` (basically using `cargo-web`, WebAssembly, and `asm.js` via Emscripten) if there were to be no compromises on performance at all (but its questionable compatibility with Phaser needs to be investigated and enquired even further so as to not go into the realm of impossibility). Time taken for conversion between the specified selected data format and a human-readable format needs to be taken into consideration and accounted for as well (especially on the client side) lest the server suffers from unnecessary performance overhead issues.

- Add Cross-Site Request Forgery (CSRF) protection to all of the endpoints (perhaps by using anti-CSRF double-submit cookies tied with a session token?), just as an enhanced security measure.

- If more specific error messages are required/needed in the future, follow [this tutorial](https://blog.logrocket.com/json-input-validation-in-rust-web-services/) to setup the JSON input validation properly.

- Avoid hardcoded path methods for files (perhaps by serving static files using [`actix-web-static-files`](https://github.com/kilork/actix-web-static-files)).

- Add automatically-generated OpenAPI/Swagger specification for the API endpoints by using [`paperclip`](https://github.com/wafflespeanut/paperclip).

- Migration from DigitalOcean Droplet to AWS, GCP or Azure since they provide a better, less troublesome and more supportive environment for CI/CD (as well as for project ownership transfer process). DigitalOcean App Platform was considered but it turned out to be not really feasible (not an open option) since based on [this documentation](https://www.digitalocean.com/docs/app-platform/#limits), App Platform applications do not have a persistent IP address (which is required/needed for the domain name resolvement).

- Implement proper authentication techniques (might want to have a look and take some insights from these crates: [`alcoholic_jwt`](https://code.tvl.fyi/tree/net/alcoholic_jwt), [`actix-identity`](https://crates.io/crates/actix-identity), [`actix-session`](https://crates.io/crates/actix-session), [`actix-web-httpauth`](https://crates.io/crates/actix-web-httpauth)). Ultimate security would be to implement multi-factor authentication (MFA), together with a single sign-on (SSO) option integrated with popular identity account providers (such as GMail, Facebook, Twitter, LinkedIn, GitHub, etc.), but for such a small and simple application, it feels and seems *very, very* overengineered.



## Acknowledgements <a name="acknowledgements"></a>

[back to top](#top)

[Credits](./images/credits.png) and props to these people who made this project possible to happen (especially during such a busy academic school term with a literal mountain of workload, sacrificing a lot of things):

- SUTD ISTD 2021 Open House Game Development Committee:
  - [Filbert Cia](https://github.com/FolkLoreee) (Project Director & Overall Coordinator)
  - [Sean Gunawan](https://github.com/naffins)
  - [Yu Nicole Frances Cabansay](https://github.com/nicolefranc)
  - [Daniel Low Yu Hian](https://github.com/nexaitch)
  - [Arissa Rashid](https://github.com/radjsh)
  - [Ho Xin Yi Felice](https://github.com/feliceho006)
  - [Clement Vimal Ravindran](https://github.com/lanvoine)
  - [James Raphael Tiovalen](https://github.com/jamestiotio)
- Samuel Seah, Deputy Manager of SUTD's Office of Marketing & Communications as this project's person-in-charge (PIC) for initiating, ideating, planning, executing and marketing/advertising the project, as well as for assisting us with certain administrative approvals
- SUTD's Office of Information Technology for helping out with the DNS records setup



Cheers!

<p align="center">&mdash;⭐&mdash;</p>
<p align="center"><i>Crafted, designed and built with ❤️ by <a href="https://github.com/jamestiotio">@jamestiotio</a> in Singapore. (ﾉ◕ヮ◕)ﾉ*:･ﾟ✧</i></p>

<p align="center"><a href="https://istd.sutd.edu.sg/"><img alt="ISTD Logo" height="150px" src="./images/istd-logo.png"></a><a href="https://www.sutd.edu.sg/"><img alt="SUTD Logo" height="150px" src="./images/sutd-logo.png"></a></p>

