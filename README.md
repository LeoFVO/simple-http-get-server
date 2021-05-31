# Pima server

HTTP GET only
<pima-root> is /tmp/pima by default

## Info

- Only provide static HTML pages at port 8080
- all static pages under common folder : <pima-root>/static
- 404 page at <pima-root>/404.html
- one thread per http request

## Configuration

    --root root folder (/tmp/pima by default)
    --ip IP served (0.0.0.0 by default)
    --port TCP port (8080 by default)
# simple-http-get-server
