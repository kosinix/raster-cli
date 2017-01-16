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
                        .output()
                        .expect("failed to execute process");


    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success());
}

#[test]
fn equal_test(){

    use std::process::Command;

    let output = Command::new("raster-cli")
                        .arg("equal")
                        .arg("tests/in/sample.jpg")
                        .arg("tests/in/sample.jpg")
                        .output()
                        .expect("failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("status: {}", output.status);
    println!("stdout: {}", stdout);
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success());
    assert_eq!("true", stdout.trim());
}


#[test]
fn fill_test(){

    use std::process::Command;

    let output = Command::new("raster-cli")
                        .arg("fill")
                        .arg("tests/in/sample.jpg")
                        .arg("tests/out/fill.png")
                        .arg("#FF0000")
                        .output()
                        .expect("failed to execute process");


    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success());
}

#[test]
fn gamma_test(){

    use std::process::Command;

    let output = Command::new("raster-cli")
                        .arg("gamma")
                        .arg("tests/in/sample.jpg")
                        .arg("tests/out/gamma.png")
                        .arg("2.2")
                        .output()
                        .expect("failed to execute process");


    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success());
}

#[test]
fn resize_test(){

    use std::process::Command;

    let output = Command::new("raster-cli")
                        .arg("resize")
                        .arg("tests/in/sample.jpg")
                        .arg("tests/out/resize.png")
                        .arg("150")
                        .arg("100")
                        .arg("fit")
                        .output()
                        .expect("failed to execute process");


    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success());
}

#[test]
fn rotate_test(){

    use std::process::Command;

    let output = Command::new("raster-cli")
                        .arg("rotate")
                        .arg("tests/in/sample.jpg")
                        .arg("tests/out/rotate.png")
                        .arg("40")
                        .arg("#FFFFFF00")
                        .output()
                        .expect("failed to execute process");


    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success());
}

#[test]
fn similar_test(){

    use std::process::Command;

    let output = Command::new("raster-cli")
                        .arg("similar")
                        .arg("tests/in/sample.jpg")
                        .arg("tests/in/watermark.png")
                        .output()
                        .expect("failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("status: {}", output.status);
    println!("stdout: {}", stdout);
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success());
}