
/*
NOTE:
  Strings are null terminated in md3 and names are maximum
  64 characters in length

*/

extern crate byteorder;

// FIXME:  Implement loading for other structures such as 

pub mod math
{

    pub struct Vec3
    {
        pub x : f32,
        pub y : f32,
        pub z : f32
    }

}


pub mod geom
{
    use math::Vec3;
    use md3::Md3Model;

    #[allow(dead_code)]
    struct StaticMesh
    {
        triangles:  Vec<Vec3>,
        indicies: Vec<i32>
    }

    #[allow(dead_code)]
    struct VertexAnimatedMesh
    {
        frames : Vec<StaticMesh>
    }

    #[allow(dead_code)]
    enum GLReadyMesh{
        VertexAnimated( VertexAnimatedMesh ),
        Static( StaticMesh ),
        Corrupted( String ) // Failed on loading
    }

    trait CreateGLReadyMesh {
        fn create_gl_ready_mesh( &mut self ) -> GLReadyMesh;
    }


    impl CreateGLReadyMesh for Md3Model
    {
        // FINISHME
        fn create_gl_ready_mesh( &mut self ) -> GLReadyMesh
        {
            // Convert to GL ready format
            if self.header.frame_count > 1 {
                // VertexAnimated mesh will be produced
                
            }else{
                // Static mesh will be produced 
                
            }
            return GLReadyMesh::Corrupted(String::from("Cannot make GL ready mesh from MD3 model!"));
        }
    }

}

#[allow(dead_code)]
pub mod md3 {

    use std::io::prelude::*;
    // use std::ffi::CString;
    use std::io::SeekFrom;
    use std::fs::File;
    use std::mem;
    use byteorder::{LittleEndian,ReadBytesExt};
    use math::Vec3;

    const MAX_QPATH : usize = 64;
    const MD3_XYZ_SCALE : f32 = 1.0/64.0;

    #[allow(dead_code)]
    pub struct Md3Header
    {
        pub ident : i32,
        pub version : i32,
        pub name : Vec<u8>,
        pub flags : i32,
        pub frame_count : i32,
        pub tags_count : i32,
        pub surface_count : i32,
        pub skin_count : i32,
        pub frames_offset : i32,
        pub tags_offset : i32,
        pub surfaces_offset : i32,
        pub eof_offset : i32,
    }

    pub struct Md3Frame
    {
        pub bounds : [Vec3; 2],
        pub local_origin : Vec3,
        pub radius : f32,
        pub name : [u8; 16]
    }

    pub struct Md3Tag
    {
        pub name : [u8; MAX_QPATH],
        pub origin : Vec3,
        pub axis : [Vec3; 3],
    }

    pub struct Md3Shader
    {
        pub name : [u8; MAX_QPATH],
        pub shader_index : i32
    }

    pub struct Md3SurfaceHeader
    {
        pub ident : i32,
        pub name  : [u8; MAX_QPATH],
        pub flags : i32,
        pub frame_count : i32,
        pub shader_count : i32,
        pub vertex_count : i32,
        pub triangle_count : i32,
        pub triangles_offset : i32,
        pub shaders_offset : i32,
        pub st_offset : i32, // Like UV but with inverted y axis
        pub xyz_normals_offset : i32,
        pub end_offset : i32,
    }

    pub struct Md3SurfaceData
    {
        pub triangles:   Vec<Md3Triangle>,
        pub shaders:     Vec<Md3Shader>,
        pub st_data:     Vec<Md3St>,
        pub xyz_normals: Vec<Md3XyzNormal>,
    }

    pub struct Md3Surface
    {
        pub header: Md3SurfaceHeader,
        pub data:   Md3SurfaceData
    }

    pub struct Md3Triangle
    {
        pub indicies : [i32 ; 3]
    }

    pub struct Md3St
    {
        st : [f32; 2]
    }

    pub struct Md3XyzNormal
    {
        pub xyz : [i16 ; 3],
        pub normal : i16
    }

    #[allow(dead_code)]
    pub struct Md3Model
    {
        pub header : Md3Header,
        pub frames : Vec<Md3Frame>,
        pub surfaces : Vec<Md3Surface>,
        pub st_buffer : Vec<Md3St>,
        pub xyz_normals : Vec<Md3XyzNormal>,
        pub shaders : Vec<Md3Shader>
    }


    // fn load_raw_struct<Type, Reader : Read + Seek >( hdr : &mut Type, in_strm : &mut Reader ) 
    // {
    //     let hdr_size = std::mem::size_of::<Type>();
    //     unsafe {
    //         let hdr_slice = slice::from_raw_parts_mut(
    //             (hdr) as *mut _ as *mut u8,
    //             hdr_size
    //         );
    //         in_strm.read_exact(hdr_slice).unwrap();
    //     }

    // }

    macro_rules! read_all_little_i32{
        ($i:ident;$($var_name:expr),+) => {
            $(($var_name = $i.read_i32::<LittleEndian>().expect(format!("FAILED LOADING FIELD OF {} STRUCURE", stringify!($s) ).as_str()));)+;

        }; 
        (($s:ident, $i:ident);$($var_name:tt),+) => {
            $(($s.$var_name = $i.read_i32::<LittleEndian>().expect(format!("FAILED LOADING FIELD OF {} STRUCURE", stringify!($s) ).as_str()));)+;
        };
    }

    macro_rules! read_all_little_i16{
        ($i:ident;$($var_name:expr),+) => {
            $(($var_name = $i.read_i16::<LittleEndian>().expect(format!("FAILED LOADING FIELD OF {} STRUCURE", stringify!($s) ).as_str()));)+;

        }; 
        (($s:ident, $i:ident);$($var_name:tt),+) => {
            $(($s.$var_name = $i.read_i16::<LittleEndian>().expect(format!("FAILED LOADING FIELD OF {} STRUCURE", stringify!($s) ).as_str()));)+;
        };
    }

    macro_rules! read_all_little_f32{
        ($i:ident;$($var_name:expr),+) => {
            $(($var_name = $i.read_f32::<LittleEndian>().expect(format!("FAILED LOADING FIELD OF {} STRUCURE", stringify!($s) ).as_str()));)+;

        }; 
        (($s:ident, $i:ident);$($var_name:tt),+) => {
            $(($s.$var_name = $i.read_f32::<LittleEndian>().expect(format!("FAILED LOADING FIELD OF {} STRUCURE", stringify!($s) ).as_str()));)+;
        };
    }




    impl Md3Header
    {

        fn read_from<R : Read + Seek>( inp: &mut R ) -> Md3Header
        {
            let mut hdr : Md3Header = unsafe { mem::zeroed() };
            hdr.name  =  vec![0 as u8;64];
            // load_raw_struct( &mut hdr, in_strm );
            hdr.version = inp.read_i32::<LittleEndian>().unwrap();
            inp.read( &mut hdr.name ).unwrap();
            let hdr_name_nul =  hdr.name.iter().skip_while(|&&x| x == 0).count(); 
            hdr.name.truncate( hdr_name_nul );

            read_all_little_i32!{
                (hdr, inp);
                flags, frame_count,
                tags_count, surface_count,
                skin_count, frames_offset,
                tags_offset, surfaces_offset,
                eof_offset
            };

            return hdr;
        }
    }

    macro_rules! assign_fields {
        ($val:expr, $struct:ident; $($field:ident)+) => {
            $($struct.$field = $val;)+ 
        }
    }

    impl Vec3
    {
        fn read_from<RType: Read >( &mut self , inp : &mut RType )
        {
            assign_fields!{
                inp.read_f32::<LittleEndian>()
                    .expect(String::from("Could not read VEC3 from stream!").as_str()), self ;
                x y z
            }
        }
    }

    impl Md3St
    {
        fn read_from<RType: Read + Seek>( inp : &mut RType, start_offset : i32,
                                          buff : &mut Vec<Md3St>, count : i32 )
        {
            inp.seek( SeekFrom::Start( start_offset as u64 ) )
                .expect("Error while seeking to ST position in MD3 file!");
            for _  in 0 ..  count {
                let mut st : Md3St = unsafe { mem::zeroed() };
                read_all_little_f32!{
                    inp;
                    st.st[0], st.st[1]
                }

                buff.push( st );
            }
        }
    }

    impl Md3Frame
    {
        fn read_from<RType: Read + Seek>( inp: &mut RType, buff : &mut Vec<Md3Frame>, count: i32 ) 
        {
            for _ in 0 .. count  {
                let mut frm : Md3Frame = unsafe { mem::zeroed() };
                frm.bounds[0].read_from( inp );
                frm.bounds[1].read_from( inp );
                frm.local_origin.read_from( inp ); 
                read_all_little_f32!( (frm,inp); radius );
                inp.read(  &mut frm.name ).unwrap();
                buff.push( frm );
            }
        }
    }

    impl Md3XyzNormal
    {

        fn read_from<RType: Read + Seek>( inp : &mut RType, start_offset : i32, buff : &mut Vec<Md3XyzNormal>, count : i32 )
        {
            inp.seek( SeekFrom::Start( start_offset as u64 ) )
                .expect("Error while seeking to XyzNormal position in MD3 file!");
            // xyz is stored as i16, they have to be scaled by a factor of 1/64
            for _ in 0 .. count {
                let mut xyzn : Md3XyzNormal  = unsafe { mem::zeroed() };
                read_all_little_i16!{
                    inp;
                    xyzn.xyz[0], xyzn.xyz[1], xyzn.xyz[2], xyzn.normal
                };
                buff.push( xyzn );
            }
        }

        fn decode_normal( &self ) -> Vec3
        {
            use std::f64::consts::PI;
            let lat = ((self.normal >> 8) & 255) as f64 * (2.0* PI) / 255.0;
            let lng = (self.normal & 255) as f64 * ( 2.0 * PI ) / 255.0;
            Vec3 {
                x: (lat.cos() *  lng.sin()) as f32,
                y:  (lat.sin()  *  lng.sin()) as f32,
                z:  lng.cos() as f32
            }
        }
    }

    impl Md3Surface
    {
        fn read_from<RType: Read + Seek>( inp: &mut RType, start_offset : i32 , buff : &mut Vec<Md3Surface>, count: i32 ) 
        {

            inp.seek( SeekFrom::Start( start_offset as u64) )
                .expect("Could not seek to surfaces offset!"); 

            for _ in 0 .. count {
                let mut surf_header : Md3SurfaceHeader = unsafe { mem::zeroed() };
                let mut surf_data   : Md3SurfaceData   = unsafe { mem::zeroed() };

                inp.read( &mut surf_header.name ).unwrap();
                read_all_little_i32!{
                    (surf_header,inp);
                    flags, frame_count, shader_count,
                    vertex_count, triangle_count, triangles_offset,
                    shaders_offset, st_offset, xyz_normals_offset,
                    end_offset
                }
                // FIXME: WE SHOULD LOAD Md3SurfaceData right now!
                // FINISHME
                Md3Triangle::read_from( inp, start_offset + surf_header.triangles_offset, &mut surf_data.triangles, surf_header.triangle_count );


                Md3XyzNormal::read_from( inp, start_offset+surf_header.xyz_normals_offset, &mut surf_data.xyz_normals,  surf_header.vertex_count );

                Md3Shader::read_from( inp, start_offset+surf_header.shaders_offset,  &mut surf_data.shaders, surf_header.shader_count );

                buff.push( Md3Surface{ header: surf_header, data: surf_data } );
            }
        }
    }

    impl Md3Shader
    {
        fn read_from<RType: Read + Seek>( inp: &mut RType, start_offset : i32, buff : &mut Vec<Md3Shader>, count: i32 ) 
        {
            inp.seek( SeekFrom::Start( start_offset as u64 ) )
                .expect("Error while seeking to Shader position in MD3 file");
            for _ in 0 .. count {
                let mut shdr : Md3Shader = unsafe { mem::zeroed() };
                inp.read( &mut shdr.name )
                    .expect("Could not read shader name within MD3 file!");
                read_all_little_i32!( (shdr, inp); shader_index );
                buff.push( shdr );
            }
        }
    }

    impl Md3Tag
    {
        fn read_from<RType: Read + Seek>( inp: &mut RType, buff : &mut Vec<Md3Tag>, count: i32 ) 
        {
            for _ in 0 .. count {
                let mut tag  : Md3Tag = unsafe { mem::zeroed() };
                inp.read( &mut tag.name ).unwrap();
                tag.origin.read_from( inp );
                tag.axis[0].read_from( inp );
                tag.axis[1].read_from( inp );
                tag.axis[2].read_from( inp );
                buff.push( tag );
            }
        }
    }

    impl Md3Triangle
    {
        fn read_from<RType: Read + Seek>( inp: &mut RType, start_offset : i32, buff : &mut Vec<Md3Triangle>, count: i32 ) 
        {
            inp.seek( SeekFrom::Start( start_offset as u64 ) ).
                expect("Could not seek into file(Triangles)!");

            for _ in 0 .. count {
                let mut tri  : Md3Triangle = unsafe { mem::zeroed() };
                {
                    read_all_little_i32!(
                        inp;
                        tri.indicies[0],
                        tri.indicies[1],
                        tri.indicies[2]
                    );
                }
                buff.push( tri );
            }
        }
    }



    impl Md3Model
    {

        pub fn load( fname : String ) -> Option<Md3Model>
        {
            let mut fin = File::open(fname).expect("Could not open MD3 model file!");
            // Find IDP3 in file and read the following header
            let ident_val = [ 73 as u8, 68, 80, 51 ];
            let mut ident_idx : usize = 0;
            let mut tmp_offset = 0;

            let mut b : [u8; 1] = [0];
            loop {
                fin.read( &mut b ).ok().unwrap();
                if ident_val[ident_idx] == b[0]{
                    ident_idx += 1;
                }
                tmp_offset += 1;
                if ident_idx == 3 {
                    tmp_offset += 1;
                    break;
                }
            }

            fin.seek( SeekFrom::Start( tmp_offset ) ).ok().unwrap();
            println!("MD3 IDENT OFFSET: {}", tmp_offset);
            let mut m = Md3Model {
                header : Md3Header::read_from( &mut fin ),
                frames : vec![],
                surfaces : vec![],
                st_buffer : vec![],
                xyz_normals : vec![],
                shaders : vec![]
            };

            fin.seek( SeekFrom::Start( m.header.frames_offset as u64) )
                .expect("Could not seek to frames offset!"); 

            Md3Frame::read_from(
                &mut fin, &mut m.frames,
                m.header.frame_count
            );

            Md3Surface::read_from(
                &mut fin, m.header.surfaces_offset, &mut m.surfaces,
                m.header.surface_count
            );

            return Some(m);
        }
    }


}





