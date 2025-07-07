# Filesystem Testing

This is a set of tests to demonstrate the ability of a filesystem to support
arbitrary POSIX behaviors. To run these checks, you need to create a directory
`mountpoint` in the root of this repository and them mount a filesystem at that
directory. An example of doing this with the `awslabs/mountpoint-s3` repository
is as follows. This assumes there is a bucket named `default-bucket` at the
specified S3 endpoint.

```
rm -rf ./mountpoint/
mkdir mountpoint

mount-s3 \
  --force-path-style \
  --allow-other \
  --allow-delete \
  --allow-overwrite \
  --endpoint-url http://localhost:9999/ \
  default-bucket \
  ./mountpoint

cargo test
```
