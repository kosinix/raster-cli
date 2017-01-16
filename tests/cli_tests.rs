#[test]
fn version_test(){

    use std::process::Command;

    let output = Command::new("raster-cli")
                        .arg("-v")
                        .output()
                        .expect("failed to execute process");


    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success());
}

#[test]
fn blend_test(){

    use std::process::Command;

    let output = Command::new("raster-cli")
                        .arg("blend")
                        .arg("tests/in/sample.jpg")
                        .arg("tests/in/watermark.png")
                        .arg("tests/out/blend.png")
                        .arg("--debug")
                        .output()
                        .expect("failed to execute process");


    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success());
}

#[test]
fn crop_test(){

    use std::process::Command;

    let output = Command::new("raster-cli")
                        .arg("crop")
                        .arg("tests/in/sample.jpg")
                        .arg("tests/out/crop.png")
                        .arg("200")
                        .arg("200")
                        .arg("top_left")
                        .arg("--debug")
                        .output()
                        .expect("failed to execute process");


    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success());
}