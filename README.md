# Ruset

## Instructions

**Build**

`cargo build --release`

**Install**

`./ruset && cargo install --path .`

**Run**

`ruset some/dir -l -y`

## Use cases

- **Perform a clean install of the `node_modules` folder on a project:** The reason for this
  is that in a **private project** there are some changes on the Design System that we need to test
  locally and that changes sometimes break the `node_modules` folder and Vite is not
  loading the dependencies correctly.

  Is that workflow what we want?... absolutely not. Is optimal? ... nope, but, this command just makes my life a little bit more easier 🥹

- **Install dependencies as `legacy`:** There are some old dependencies in the project that
  we need to handle, this is actually a WIP but there are another priorities and we are taking
  care of this issue in the free time... this is just an improvement for **one specific case**

- **Remove yalc local changes:** This is related to the first point. This feature comes handy when a new
  DS breaking change is available

<br>

---

**How I expect the command to work**

```sh
ruset path/to/dir (opts)
```

If no path provided search the `node_module` folder inside the current directory (this is going to be my goto almost all the time)

```sh
ruset (opts)
```

---

<br>

### TODO

- [ ] Improve the `--help` command
- [x] ~~Manage special use cases, this means that the execution of the command should work with the args specified~~
