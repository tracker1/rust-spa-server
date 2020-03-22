# spa-server

This is meant to be an SPA application server written in rust.  It will have the following features:

* Will serve static content from './static'
* Will support E-Tag
* Will support if-modified-since
* Will deliver pre-compressed brotli or gzipped files

This server will not support additional features.  It is a static content server meant to be used in a containerized environment.

## Docker usage

For docker builds, there will be two versions, both will use root (`/`) as the working directory, delivering content from `/static` as a result.

1. default - will be a bare container, the executable will be `/spa-server`.
2. busybox - will include a busybox environment with the executable will be at `/bin/spa-server`.  This environment will allow for minimal scripting in order to pre-run components as necessary.
