# spa-server

This is meant to be an SPA application server written in rust.  It will have 
the following features:

* Will serve static content from './static'
* Will support E-Tag
* Will support if-modified-since
* Will deliver pre-compressed brotli or gzipped files

This server will not support additional features.  It is mostly meant as a 
content server meant to be used in a containerized environment.

## Docker usage

For docker builds, there will be two versions, both will use root as the 
working directory.

1. default - will be a bare container, the executable will be `/spa-server` and 
will deliver content from `/static`.
3. busybox - will include a busybox environment with the executable in 
`/bin/spa-server` and will deliver content from `/static` by default.
