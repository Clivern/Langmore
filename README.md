<p align="center">
    <img src="https://raw.githubusercontent.com/Clivern/Langmore/main/static/12.jpeg" width="75%" />
    <h3 align="center">Langmore</h3>
    <p align="center">A KV Store Based On Write-Ahead Log</p>
    <p align="center">
        <a href="https://github.com/Clivern/Langmore/actions"><img src="https://github.com/Clivern/Langmore/actions/workflows/build.yml/badge.svg"></a>
        <a href="https://github.com/Clivern/Langmore/releases"><img src="https://img.shields.io/badge/Version-v0.3.0-green.svg"></a>
        <a href="https://github.com/Clivern/Langmore/blob/main/LICENSE"><img src="https://img.shields.io/badge/LICENSE-MIT-green.svg"></a>
    </p>
</p>


## Design of Langmore

`Langmore` uses a lot of principles from log-structured file systems and draws inspiration from a number of designs that involve log file merging. It essentially is just a directory of append-only (log) files with a fixed structure and an in-memory index holding the keys mapped to a bunch of information necessary for point lookups - referring to the entry in the datafile.

### Datafiles

They re append-only log files that hold the KV pairs along with some meta-information. A single `Langmore` instance could have many datafiles, out of which just one will be active and opened for writing, while the others are considered immutable and are only used for reads.
<br/>
<p align="center">
    <img src="https://raw.githubusercontent.com/Clivern/Langmore/main/static/datafiles.png?v=0.1.0" width="90%" />
</p>


Each entry in the datafile has a fixed structure illustrated above and it stores `CRC`, `timestamp`, `key_size`, `value_size`, `actual_key`, and the `actual_value`. All the write operations - create, update and delete - made on the engine translates into entries in this active datafile. When this active datafile meets a size threshold, it is closed and a new active datafile is created. when closed (intentionally or unintentionally), the datafile is considered immutable and is never opened for writing again.

### KeyDir

It is an in-memory hash table that stores all the keys present in the Langmore instance and maps it to the offset in the datafile where the log entry (value) resides; thus facilitating the point lookups. The mapped value in the Hash Table is a structure that holds `file_id`, `offset`, and some meta-information like timestamp, as illustrated below.

<p align="center">
    <img src="https://raw.githubusercontent.com/Clivern/Langmore/main/static/keydir.png?v=0.1.0" width="90%" />
</p>


## Deployment

Build the project with the following command

```bash
$ make build
```

or install with cargo

```bash
$ cargo install langmore
```

Define the configs and run the `Langmore` binary.

```bash
export HOSTNAME=127.0.0.1:8080
export STORAGE_DIR=/etc/langmore

$ ./target/debug/langmore
```


## Usage

You can use `netcat` to interact with `Langmore`

```bash
$ nc 127.0.0.1 8080
```


## Versioning

For transparency into our release cycle and in striving to maintain backward compatibility, Langmore is maintained under the [Semantic Versioning guidelines](https://semver.org/) and release process is predictable and business-friendly.

See the [Releases section of our GitHub project](https://github.com/clivern/langmore/releases) for changelogs for each release version of Langmore. It contains summaries of the most noteworthy changes made in each release.


## Bug tracker

If you have any suggestions, bug reports, or annoyances please report them to our issue tracker at https://github.com/clivern/langmore/issues


## Security Issues

If you discover a security vulnerability within Langmore, please send an email to [hello@clivern.com](mailto:hello@clivern.com)


## Contributing

We are an open source, community-driven project so please feel free to join us. see the [contributing guidelines](CONTRIBUTING.md) for more details.


## License

© 2022, clivern. Released under [MIT License](https://opensource.org/licenses/mit-license.php).

**Langmore** is authored and maintained by [@Clivern](http://github.com/clivern).
