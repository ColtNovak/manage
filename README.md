# Manage 

`manage` is a simple CLI password manager I wrote in rust while i was in class.

---

## Features
- **Store info**: 
- **Retrieve info**: View saved usernames and passwords.
- **Simple**: Credentials are stored in plain text files (for now) 

---


# installation
 **1. Clone the Repository**:
   ```
 git clone https://github.com/COltNovak/manage.git && cd manage
````
 **2. Compile project**
  ```
  rustc manage.rs
  ```
**3. install binary**
```
sudo mv manage /usr/local/bin
```
## USAGE
To make a new password type:
```
manage new <SITE NAME>
```
To find and retreive an existing password type:
```
manage <SITE NAME>
```
### TODO LIST
- encryption of some kind
- password generator
- strength checker
