# Bday

Birthday tracker.

### Compiling

You can compile it using `cargo` or install [baker](https://github.com/rv178/baker) and compile it like this:

```
bake setup
bake
```

A binary will be copied to `./bin/bday`

### Usage

#### List everyone's birthdays

```
bday list
```

#### Add a person

- Birthday should be in `day-month-year` format. eg: `17-08-2006`

```
bday add [name] [birthday]
```

#### Remove a person

```
bday rm [id]
```
