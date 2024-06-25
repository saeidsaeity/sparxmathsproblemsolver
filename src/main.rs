struct ArcLengths{
    arc_length:f32,
    large_arc_length:f32,
}
fn arc_calculator(radius:f32,angle:f32)-> ArcLengths{
    let pi = std::f32::consts::PI;
    let arc_length = 2.0*radius*pi*(angle/360.0) ;
    let large_arc_length= 2.0*radius*pi*((360.0-angle)/360.0);
    ArcLengths{arc_length,large_arc_length}

}
struct CircleAreas{
    arc_area: f32,
    rest_of_circle:f32,
}
fn arc_area (radius:f32,angle:f32)->CircleAreas{
    let pi = std::f32::consts::PI;
    let arc_area =pi*radius*radius*(angle/360.0);
    let rest_of_circle =pi*radius*radius*((360.0-angle)/360.0);
   CircleAreas{arc_area,rest_of_circle}
}
fn prism_triangle (height:f32,depth:f32,width:f32)->f32{
   
    let triangle_prism = depth*width*3.0 + height*width;
    triangle_prism

}
fn cuboid (height:i32,width:i32,depth:i32)->i32{
    let cuboid_area = height*width*2+ height*depth*2+width*depth*2;
    cuboid_area
}
#[derive(Debug)]
struct PentagonAreas{
    one_face:f32,
    all_faces:f32,
}
fn pentagon_prism(height:i32,width:i32,pentagon:f32)->PentagonAreas{
    let heightf32= height as f32;
    let widthf32=width as f32;
    let one_face = heightf32 * widthf32;
    let all_faces = heightf32*widthf32 *5.0 + 2.0*pentagon;
    PentagonAreas{one_face,all_faces}
    
}
fn triangle_prism_odd (tb:i32,theight:i32,thypotnuse:i32,recheight:i32)->i32{
    let cuboid_area = recheight*thypotnuse + tb*recheight + theight*tb+theight*recheight;
    cuboid_area
}
fn main() {
    let arcresult = arc_calculator(11.0,84.0);
    println!("arc perimeter {}, large arc perimter {}",arcresult.arc_length,arcresult.large_arc_length);
    let result=arc_area(16.0,39.0);
    println!("arc area {},{}",result.arc_area,result.rest_of_circle);
    println!("equilateral prism {}", prism_triangle(5.6,11.6,6.5));
    println!("cuboid area {}",cuboid(12,7,9));
    let pentagonres = pentagon_prism(11,3,15.5);
    println!("{:#?}",pentagonres);
    println!("prism surface area {}",triangle_prism_odd(12,9,15,16));
}
