# Overdue! (Database Server Backend API)

<p align="center"><img alt="Overdue! Logo" width="420px" src="./images/overdue-logo.png"></p>

![POWERED BY: ISTD SUTDENTS](https://img.shields.io/badge/powered%20by-istd%20SUTDents-73af44?style=for-the-badge&labelColor=d7ef32) ![COVERAGE: 43%](https://img.shields.io/badge/coverage-43%25-orange?style=for-the-badge)

[![Build, Run Tests & Deploy](https://img.shields.io/github/workflow/status/jamestiotio/overdue_backend/Build%2C%20Run%20Tests%20%26%20Deploy?label=Build%2C%20Run%20Tests%20%26%20Deploy&logo=github&style=for-the-badge)](https://github.com/jamestiotio/overdue_backend/actions/workflows/main.yaml) [![Security Audit](https://img.shields.io/github/workflow/status/jamestiotio/overdue_backend/Security%20Audit?label=Security%20Audit&logo=github&style=for-the-badge)](https://github.com/jamestiotio/overdue_backend/actions/workflows/audit.yaml) [![Daily Security Audit](https://img.shields.io/github/workflow/status/jamestiotio/overdue_backend/Daily%20Security%20Audit?label=Daily%20Security%20Audit&logo=github&style=for-the-badge)](https://github.com/jamestiotio/overdue_backend/actions/workflows/daily-audit.yaml)

Backend Database Server Code of `Overdue!` (SUTD Fablab ISTD Game) for Open House 2021.

Game obviously inspired by [Overcooked!](https://www.team17.com/games/overcooked/).

Frontend code repository for the [Phaser](https://phaser.io/)-based public-facing web game client is accessible [here](https://github.com/nicolefranc/sutd-fablab-game) (might be private).

While they are still up, the front-facing frontend game client can be accessed [here](https://sutd-fablab-game.netlify.app/) (the public-facing one can be seen either [here](https://openhouse.sutd.edu.sg/) or [here](https://overdue.sutd.edu.sg/)) and the backend game server can be accessed [here](https://sutdoverdue.dev/).

For players who contributed to the game's final goal of total materials collected, they will earn this badge:

<p align="center"><a href="https://au.badgr.com/public/badges/iQfJV6FcSP2cMQlfVyXBug"><img alt="SUTD Overdue! Game Contributor" width="100px" src="https://media.au.badgr.com/uploads/badges/6ce4a23d-ff9f-4d57-bc92-c24724cf569d.svg"></a></p>



## Justification of Design Decisions

> Server performance and data safety/security are the *sine qua non* for this project. To be very honest, this is a ridiculously super over-engineered no-nonsense implementation for *...ahem... [serious business purposes](https://github.com/EnterpriseQualityCoding/FizzBuzzEnterpriseEdition)* (although due to time constraint, this project is not really to that level of no-nonsense... yet?). But I consider it a good thing. Welp. 🤷 Fight me. ᕕ( ᐛ )ᕗ

The reason we utilized [Rust](https://www.rust-lang.org/) is due to its performance, reliability, memory and thread safety, easier concurrency and zero-cost abstractions. It also addresses and solves a lot of pain points present in many other programming languages (with a couple of advantages over/compared to languages such as JavaScript, C/C++ and Go). Its strong and static type system allows us to conduct some initial input validation even before any data is being passed on to inner/deeper functions, ensuring better security of our backend server application. Rust's performance in terms of speed is even comparable to `C++ g++` and `C gcc` (source [here](https://benchmarksgame-team.pages.debian.net/benchmarksgame/which-programs-are-fastest.html)). The reason we utilized [`actix-web`](https://actix.rs/) is because it is production-ready, powerful, pragmatic, type-safe and extremely fast/performant (check out its performance on [TechEmpower Framework Benchmarks](https://www.techempower.com/benchmarks/), keeping in mind the caveats of such benchmarks). It is also the top-performing web framework written in Rust, thereby allowing us to achieve high performance and high security. The reason we utilized [`tokio-postgres`](https://github.com/sfackler/rust-postgres) is because it utilizes the same Tokio runtime used by Actix. As such, this allows us to achieve this synergy of mutual reinforcement and compatibility for the sake of data manipulation, data formatting and data types. It also has awesome community support, which is greatly helpful since there are many helper crates that were able to improve our productivity rate for the sake of delivering the MVP (minimum viable product) of this backend server as we were able to focus better and more on creating the main business logic of this application instead of having to deal with other more menial tasks such as struct data type conversions and complicated manual input validation.

Developing this server code prototype was quite enjoyable, even though the timeline was pretty intensive. Maybe I will become a Rustacean one day. 😌



## Setup Instruction

1. Download and install Rust with its toolchain (including `rustc`, `cargo` and `rustup`) from [here](https://www.rust-lang.org/tools/install). After that, optionally run these commands to use Clippy:

    ```cmd
    > rustup self update
    > rustup component add clippy
    ```
    
2. If you are on Ubuntu/Linux or MacOS, ensure that you have the usual C/C++ toolchain, OpenSSL and PostgreSQL installed. Follow the instructions [here](https://www.postgresql.org/download/) as per normal, run these commands:

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

3. Ensure that only access by `localhost` is allowed for PostgreSQL. The following `pg_hba.conf` configuration should be sufficient:

    ```
    # TYPE  DATABASE        USER            ADDRESS                 METHOD
    local   all             all                                     trust
    host    overdue         overdue       192.168.1.1/32            md5
    host    overdue         overdue       172.17.0.0/16             md5
    host    all             all           0.0.0.0/0                 reject
    ```

4. Alternatively, you can set up the PostgreSQL database service by using Docker so as to isolate the container's network from the host's network. Install [Docker Engine](https://docs.docker.com/engine/install/), [Docker Compose](https://docs.docker.com/compose/install/) and [PostgreSQL](https://www.postgresql.org/download/) first. Then, run this command:

    ```cmd
    > docker-compose up -d
    ```

    Keep in mind that the system's `postgresql` service should be disabled since the `postgres` Docker service might be in conflict and become unable and fail to bind to the `0.0.0.0:5432` address/port (since the address is already in use). This can be done by running this command:

    ```console
    $ sudo systemctl stop postgresql.service
    ```

5. To set up the database, run this command:

    ```cmd
    > psql -h 127.0.0.1 -p 5432 -U overdue overdue < database.sql
    ```

6. Enable security features for the backend server code. There are two things: firstly, follow the instructions [here](https://letsencrypt.org/getting-started/) and [here](https://certbot.eff.org/instructions) to generate the SSL certificate for HTTPS support (at least TLS v1.2) using Certbot with Let's Encrypt as its Certificate Authority and put the certificate into the appropriate folder directory (this would require a domain name for the backend server so go and obtain one from your nearest domain name registrar). For our case, we utilize Namecheap as our domain name registrar (remember to enable WhoisGuard, PremiumDNS and DNSSEC). Add the corresponding DNS Host Records:

    | Type | Host | Value | TTL |
    | --- | --- | --- | --- |
    | A Record | `@` | `<DigitalOcean Droplet's IPv4 Address>` | 60 min |
    | A Record | `www` | `<DigitalOcean Droplet's IPv4 Address>` | 60 min |
    | CAA Record | `@` | `0 issue letsencrypt.org` | 60 min |

    Secondly, get a domain name for the game client as well for the purposes of the CORS policy. Preferably, the domains should have TLD-level HSTS enabled (i.e. included in the HSTS preload secure namespace list, such as `.APP` or `.DEV` domains) so as to force secure HTTPS connections through browsers by default, which actually does not affect Let's Encrypt ACME Challenge verification process (HTTP connections would not be automatically redirected to HTTPS since [this PR](https://github.com/petertrotman/actix-web-middleware-redirect-https/pull/4) is not merged yet). Regarding prevention of XSS attacks, the `Content-Security-Policy` header should be automatically set by `actix-web` and the TLS implementation for this back-end server. The front-end game client would include the corresponding appropriate `Content-Security-Policy` header as well. By default, the backend server code will be rate-limited and will possess enough necessary input validation and verification (any malformed, incompatible or invalid input data will be automatically rejected). For additional security, a load balancer could also be deployed to mitigate DDoS attacks. These are the settings for the DigitalOcean Cloud Firewall:

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
    
    For testing purposes, you can generate a self-signed certificate-key pair of `cert.pem` and `privkey.pem` by running these commands (on Ubuntu/Linux or a UNIX-based system equipped with OpenSSL):

    ```console
    $ touch ~/.rnd
    $ dd if=/dev/urandom of=~/.rnd bs=256 count=1
    $ sudo openssl req -x509 -newkey rsa:4096 -nodes -keyout tls/privkey.pem -out tls/cert.pem -days 365 -subj '/CN=localhost'
    ```

7. Optionally, follow [this tutorial](https://www.digitalocean.com/community/tutorials/how-to-securely-manage-secrets-with-hashicorp-vault-on-ubuntu-16-04) to set up HashiCorp Vault for the DigitalOcean VPS for the purpose of storing and accessing/reading environment variables and credentials securely. Remember to use TLS certificates, enable Consul encryption and enable ACLs to make it production-ready. Alternatively, set the appropriate environment variables and credentials for the backend app server (such as the TLS certificates and the PostgreSQL database credentials).

8. For linting and testing, run these commands:

    ```cmd
    > cargo clippy
    > cargo test --all-features
    ```

9. For development, run these commands:

    ```cmd
    > cd overdue_backend\
    > cargo install cargo-watch
    > cargo build
    > cargo watch -x run
    ```

10. For release, you can install the binary by running `cargo install --bin overdue_backend --path .` or by running these commands (by default, the executable is placed in the `./target/release` folder):

    ```cmd
    > cd overdue_backend\
    > cargo install cargo-deb
    > cargo build --bin overdue_backend --release
    > cargo run --release
    ```

    We are using GitHub Actions for Continuous Integration and Continuous Delivery. Alternatively, you can follow this [tutorial](https://www.digitalocean.com/community/tutorials/how-to-install-and-configure-drone-on-ubuntu-20-04) to run tests using Drone CI. We are using SSH as our method of deployment since the alternative would be by using the `doctl` CLI, which is sort of more dangerous/risky in terms of security/safety since instead of potentially "exposing" the SSH key to a single Droplet instance, we might "expose" a whole DigitalOcean PAT API key with read and write permissions in my DigitalOcean account (with the tradeoff of being slightly less robust due to the hardcoded absolute paths to the executable binaries). As such, please be reminded to specify the specific SSH `id_rsa` private keyfile with no passphrase (by using `-i ~/.ssh/id_rsa`), specify the specific SSH `known_hosts` file (by using `-o UserKnownHostsFile=~/.ssh/known_hosts` to avoid the warning of non-establishable ECDSA key fingerprint authenticity of the host) and configure the `$PATH` environment variables accordingly so as to be able to properly run any executable binaries since SSH is a non-interactive shell (perhaps by using absolute paths or by installing the needed executables using the official Ubuntu's package manager `apt`). To allow Git to checkout, clone, pull and merge this repository, we utilize a [read-only deploy key](https://github.blog/2015-06-16-read-only-deploy-keys/) installed on the target server machine (instructions specified [here](https://docs.github.com/en/developers/overview/managing-deploy-keys#deploy-keys)).

11. For cleanup, stop the running executable file/process and run this command:

    ```cmd
    > docker-compose down
    ```

    For the purposes of final statistics and to assist in distributing the virtual custom badges to the different players and contributors, we can get all the unique emails in the leaderboard and output it into a CSV file by running this command:

    ```console
    $ psql -h 127.0.0.1 -p 5432 -U overdue -d overdue -c "COPY (SELECT DISTINCT email FROM leaderboard) TO STDOUT WITH CSV HEADER" > emails.csv
    ```

    An alternative command would be this (might either differ slightly in terms of formatting or produce an identical output for our very specific case):

    ```console
    $ psql -h 127.0.0.1 -p 5432 -U overdue -d overdue -c "SELECT DISTINCT email FROM leaderboard" --csv > emails.csv
    ```



## API Endpoints

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

For the `/submit_score` and `/get_materials` endpoints, all methods/entries/specifications/keys/parameters are REQUIRED (no optional ones). For the `/get_leaderboard` endpoint, the `max_entries` and `difficulty` query parameters are optional. An HTTPS connection is mandatory/compulsory. Any non-HTTPS connections, any connections from any other domains (as indicated by the corresponding CORS policy) and any requests that does not comply within the stated specifications will be rejected/ignored. For all endpoints, the `Host` header must be specified with the correct value, otherwise a `404 Not Found` error will be returned. A `200 OK` HTTP response code should be received if everything is executed successfully. If there are any errors, the corresponding endpoint would return another error code (such as `400 Bad Request` or `500 Internal Server Error`), along with the corresponding error message content describing what error specifically has occurred (which might be useful for debugging purposes).

If a wrong endpoint resource is specified, a `404 Not Found` error will be returned. Else, if a wrong method is used, a `405 Method Not Allowed` error will be returned (due to the method guards being implemented). Else, if the rate limit is exceeded for a particular IP address, a `429 Too Many Requests` error will be returned. Otherwise, if an error is encountered, this will be the response JSON data format:

```json
{
  "code": <some-response-code>,
  "message": "<some-error-message>",
  "error": "<some-error-description>"
}
```

The error handler is set up to avoid unwanted panics and the error message is purposefully vague and not too specific so as to avoid prying eyes from figuring out and reverse-engineering the specific cause of error and forming a malicious payload that fits within the reasonable limits of our application.



## Future Development Notes

- For an extreme level of optimization, might want to consider using [Protocol Buffers](https://developers.google.com/protocol-buffers) instead of JSON so as to guarantee type-safety, prevent schema violations, provide simpler data accessors, have a smaller size of data being transferred around and enjoy faster serialization/deserialization not just from the server side, but also even from the client side. Consider using [`actix-protobuf`](https://github.com/actix/actix-extras/tree/master/actix-protobuf) when there is more community support for integration and compatibility between the Protobuf data format, `actix-web`, `tokio-postgres` and PostgreSQL as a whole. Possible to optimize even further to using [HDF5](https://www.hdfgroup.org/solutions/hdf5/) (use [this](https://github.com/aldanor/hdf5-rust)), [MessagePack](https://msgpack.org/), [CBOR](https://cbor.io/), [FlatBuffers](https://google.github.io/flatbuffers/) or even all the way to [Cap'n Proto](https://capnproto.org/) (use [this](https://github.com/capnproto/capnproto-rust)). Other less-known serializers might not have enough community or compatibility support for our real-life application setup (such as [Apache Thrift](https://thrift.apache.org/), [`bitsery`](https://github.com/fraillt/bitsery), [YAS](https://github.com/niXman/yas) or [`zpp`](https://github.com/eyalz800/serializer)). A hand-written serializer might be the best, but it will take too much time to write (unfeasible due to the given tight timeframe). Might want to also convert the frontend client component to using `yew` (basically using `cargo-web`, WebAssembly, and `asm.js` via Emscripten) if there were to be no compromises on performance at all (but its questionable compatibility with Phaser needs to be investigated and enquired even further so as to not go into the realm of impossibility). Time taken for conversion between the specified selected data format and a human-readable format needs to be taken into consideration and accounted for as well (especially on the client side) lest the server suffers from unnecessary performance overhead issues.

- Add Cross-Site Request Forgery (CSRF) protection to all of the endpoints (perhaps by using anti-CSRF double-submit cookies tied with a session token?), just as an enhanced security measure.

- If more specific error messages are required/needed in the future, follow [this tutorial](https://blog.logrocket.com/json-input-validation-in-rust-web-services/) to setup the JSON input validation properly.

- Avoid hardcoded path methods for files.

- Add a favicon image to the `/favicon.ico` route endpoint.

- Migration from DigitalOcean Droplet to AWS, GCP or Azure since they provide a better, less troublesome and more supportive environment for CI/CD (as well as for project ownership transfer process).



## Acknowledgements

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


