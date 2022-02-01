# cloud-meta - cloud metadata client

cloud-meta provides async cloud metadata clients for the
[Amazon][amazon], [Azure][azure], [Google][google], and
[Oracle][oracle] cloud platforms.

[examples/probe.rs](examples/probe.rs) demonstrates use of the
high-level `instance` method to probe which cloud the program is
running in. Metadata clients also provide a low level `get` method to
fetch arbitrary metadata as `Bytes`.

[amazon]: https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-instance-metadata.html
[azure]:  https://docs.microsoft.com/en-us/azure/virtual-machines/linux/instance-metadata-service
[google]: https://cloud.google.com/compute/docs/metadata/
[oracle]: https://docs.oracle.com/en-us/iaas/Content/Compute/Tasks/gettingmetadata.htm
