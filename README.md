<p align="center">
    <img src="https://www.rust-lang.org/logos/rust-logo-512x512-blk.png" width="20%" />
    <h3 align="center">DWAL</h3>
    <p align="center">A KV Store Based On Write-Ahead Log</p>
    <p align="center">
        <a href="https://github.com/Clivern/Dwal/actions"><img src="https://github.com/Clivern/Dwal/actions/workflows/build.yml/badge.svg"></a>
        <a href="https://github.com/Clivern/Dwal/releases"><img src="https://img.shields.io/badge/Version-v0.1.3-green.svg"></a>
        <a href="https://github.com/Clivern/Dwal/blob/main/LICENSE"><img src="https://img.shields.io/badge/LICENSE-MIT-green.svg"></a>
    </p>
</p>


## Deployment

```bash
#
```

## Design of DWAL

DWAL uses a lot of principles from log-structured file systems and draws inspiration from a number of designs that involve log file merging. It essentially is just a directory of append-only (log) files with a fixed structure and an in-memory index holding the keys mapped to a bunch of information necessary for point lookups - referring to the entry in the datafile.

### Datafiles

They re append-only log files that hold the KV pairs along with some meta-information. A single Bitcask instance could have many datafiles, out of which just one will be active and opened for writing, while the others are considered immutable and are only used for reads.
<br/>
<p align="center">
    <img src="/static/datafiles.png?v=0.1.3" width="90%" />
</p>
<br/>
Each entry in the datafile has a fixed structure illustrated above and it stores `CRC`, `timestamp`, `key_size`, `value_size`, `actual_key`, and the `actual_value`. All the write operations - create, update and delete - made on the engine translates into entries in this active datafile. When this active datafile meets a size threshold, it is closed and a new active datafile is created. when closed (intentionally or unintentionally), the datafile is considered immutable and is never opened for writing again.

### KeyDir

It is an in-memory hash table that stores all the keys present in the DWAL instance and maps it to the offset in the datafile where the log entry (value) resides; thus facilitating the point lookups. The mapped value in the Hash Table is a structure that holds file_id, offset, and some meta-information like timestamp, as illustrated below.
<br/>
<p align="center">
    <img src="/static/keydir.png?v=0.1.3" width="90%" />
</p>


## Versioning

For transparency into our release cycle and in striving to maintain backward compatibility, Dwal is maintained under the [Semantic Versioning guidelines](https://semver.org/) and release process is predictable and business-friendly.

See the [Releases section of our GitHub project](https://github.com/clivern/dwal/releases) for changelogs for each release version of Dwal. It contains summaries of the most noteworthy changes made in each release.


## Bug tracker

If you have any suggestions, bug reports, or annoyances please report them to our issue tracker at https://github.com/clivern/dwal/issues


## Security Issues

If you discover a security vulnerability within Dwal, please send an email to [hello@clivern.com](mailto:hello@clivern.com)


## Contributing

We are an open source, community-driven project so please feel free to join us. see the [contributing guidelines](CONTRIBUTING.md) for more details.


## License

© 2022, clivern. Released under [MIT License](https://opensource.org/licenses/mit-license.php).

**Dwal** is authored and maintained by [@Clivern](http://github.com/clivern).
