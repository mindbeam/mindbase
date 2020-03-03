fn main() -> Result<(), std::io::Error> {
    // let bytes = marshal::dumps(py, doc, marshal::VERSION)?;
    // println!("{:?}", bytes);

    // for token in foo.iter() {
    //     println!("{:?}", token);
    // }

    //     let activators = PyModule::from_code(
    //         py,
    //         "
    // def relu(x):
    //     return max(0.0, x)

    // def leaky_relu(x, slope=0.01):
    //     return x if x >= 0 else x * slope
    // ",
    //         "activators.py",
    //         "activators",
    //     )?;
    //     let relu_result: f64 = activators.call1("relu", (-1.0,))?.extract()?;
    //     assert_eq!(relu_result, 0.0);
    //     let kwargs = [("slope", 0.2)].into_py_dict(py);
    //     let lrelu_result: f64 = activators
    //         .call("leaky_relu", (-1.0,), Some(kwargs))?
    //         .extract()?;
    //     assert_eq!(lrelu_result, -0.2);

    Ok(())
}
