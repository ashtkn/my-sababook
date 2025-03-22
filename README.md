# My repo for SaBA (Sample Browser Application)

Original repo: [https://github.com/d0iasm/sababook](https://github.com/d0iasm/sababook)

## Run

```sh
# On shell
python3 -m http.server 8000
./run_on_wasabi.sh

# On QEMU
saba

# Then go to:
# - http://host.test:8000/test.html
# - http://host.test:8000/test1.html
# - http://host.test:8000/test2.html
# - http://example.com
```

## Test

```sh
# Testing saba_core
cd saba_core
cargo test
```

## Screenshot

![Screenshot](screenshot.png)
