# Climate Server
Climate Server is a server application that can be used to collect and aggregate information from temperature and humidity sensors and report on their current status and provide a graph of the last seven days of activity.
## Installation
To install the Climate Server, either download a precompiled binary from the Releases page or follow these steps:
1. Have rust installed
2. Clone the repository from GitHub.
3. Enter into the repository in a terminal
4. Run the `cargo build --target [YOUR_TARGET] -r` command (the default target is set to "x86_64-unknown-linux-musl" for use with Alpine on x86_64 hardware)
## Usage
To run the application, set the required environment variables and run the application.
### Environment Variables
To set the environment files, there are two options: The dotenv file and cli arguments. 
#### Variables
| Variable | Default | Required |
|    ---   |   ---   |    ---   |
| SQL_HOST |  mysql  |    No    |
| SQL_PORT |  3306   |    No    |
| SQL_USER |  None   |    yes   |
| SQL_PASS |  None   |    yes   |
| SQL_DB   |  None   |    yes   |
|WEB_ADDRESS| 0.0.0.0|    no    |
| WEB_PORT |  3000   |    no    |
#### DOTENV
To use dotenv, simply create a file `.env` and add the key=value pairs, ex:
```dotenv
SQL_USER=climate_user
SQL_PASS=plaintext_password
SQL_HOST=localhost
SQL_DB=climate
```
#### CLI Arguments
The other way to add environment variables is to specify them in the command line. This differs depending on your platform, but generally on linux it is specified like this:
```bash
SQL_USER="climate_user" SQL_PASS="plaintext_password" SQL_HOST="localhost" SQL_DB="climate" climate-server
```
and in PowerShell you specify it like this:
```powershell
$Env:SQL_USER="climate_user"; $env:SQL_PASS="plaintext_password"; $Env:SQL_HOST="localhost"; $Env:SQL_DB="climate"; climate-server.exe
```
## Contributing
To contribute to Climate Server, follow these steps:
1. Fork the repository from GitHub.
2. Create a new branch.
3. Make your changes and commit them to the new branch.
4. Submit a pull request.
## License
Climate Server is released under the Apache 2.0 License. See the LICENSE file for more details.