# Verus

Verus is a proof-of-concept for using the same validation code on the front
end and backend of a web application. It uses Rust compiled to web assembly
on the frontend, and Ruby's FFI library to bind to a dynamic library also
compiled from the Rust code.

## Installation

At the moment Verus makes somewhat hacky use of a shell script and Docker in
order to be portable. Make sure you have Docker installed and have cloned and
CDd into the project directory, then run

```
./build-with-docker
```

That will take a few minutes. You should now be able to run the following
command in one tab:

```
docker run -p 4567:4567 verus:latest
```

and in another tab,

```
cd www
npm start
```

Now if you open a browser and visit http://localhost:8080, you should see a
very terrible looking webpage, but it should be running the same validation
code on the frontend and backend (though with different glue code).
