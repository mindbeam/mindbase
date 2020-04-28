
# Instructions to run Jupyter notebooks

```bash
#Install jupyterlab
pip install jupyterlab

# Install rust kernel
cargo install evcxr_jupyter
evcxr_jupyter --install

# cd into this directory and start the notebook server
jupyter-notebook

# a new page in your browser should open, and you may select one of the notebooks here to run
```

### Additional information about evcxr jupyter kernel setup
https://github.com/google/evcxr/blob/master/evcxr_jupyter/README.md
