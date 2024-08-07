- RSA
  create rsa private key:
  ```
  openssl genrsa -out rsa-private-key.pem 2048
  ```
  
  display rsa private key:
  ```
  openssl rsa -in rsa-private-key.pem -text -noout
  ```
- Create self signed key
  ```
  openssl genrsa -des3 -passout pass:x -out server.pass.key 2048
  openssl rsa -passin pass:x -in server.pass.key -out server.key
  # writing RSA key
  rm server.pass.key
  openssl req -new -key server.key -out server.csr
  openssl x509 -req -days 365 -in server.csr -signkey server.key -out server.crt
  ```
-
- Specify password:
  ```
  openssl aes-256-cbc -in some_file.enc -out some_file.unenc -d -pass pass:somepassword
  openssl aes-256-cbc -in some_file.enc -out some_file.unenc -k somepassword
  
  -pass:
  env:somevar to get the password from an environment variable
  file:somepathname to get the password from the first line of the file at location pathname
  fd:number to get the password from the file descriptor number.
  stdin to read from standard input
  
  ```
- Benchmark speed:
  ```
  openssl speed des-ede3
  
  ```