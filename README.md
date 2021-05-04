# gasoline

learing diesel.

build the container:

```sh
docker build -f dev.Dockerfile . -t gasoline
```

run it

make sure you have mysql running or run it with docker

```sh
docker run --name some_mysql -p 3306:3306 -e MYSQL_ROOT_PASSWORD=gasoline mysql
```

get dburl from example env file.

```sh
# build db url
source .env
echo $DATABASE_URL

**TODO:** simplify env stuff.

#setup
diesel setup --database-url $DATABASE_URL

#migrations
diesel migration run --database-url $DATABASE_URL
```

run native

```
cargo run
```

run as container

```sh
docker run --rm gasoline
```

connect native mysql client

```sh
#password in example is gasoline
mysql -h 127.0.0.1 -P 3306 -uroot -p
```

inspect

```
mysql> show databases;
mysql> use gasoline;
mysql> describe pet;
```
