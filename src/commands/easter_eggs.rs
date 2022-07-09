/*======================*/
/*      Easter Eggs     */
/*======================*/

use rand::{thread_rng, Rng};

use crate::memory::{Globals, Args};



pub fn badduck(_: &mut Globals, _: Args)
{
    let badduck  = "
                ,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
                ,,,,,,,,,,,,,,,,,@....,@@,,,,,,,,,,,,,,,,,,,,,,,,,
                ,,,,,,,,,,,,,,#...........@*,,,,,,,,,,,,,,,,,,,,,,
                ,,,,,,,,,,,,,,(@.......@....@,,,,,,,,,,,,,,,,,,,,,
                ,,,,,,,,,,,,,@,,.@@@...,**.,@,,,,,,,,,,,,,,,,,,,,,
                ,,,,,,,,,,,,,,@@@@@@@@.....@,,,,,,,,,,,,,,,,,,,,,,
                ,,,,,,,,,,,,,,,@...........@,..@,@.@,,,,,,,,,,,,,,
                ,,,,,,,,,,,,,,,@...,........,,@,&...@,,,,,,,,,,,,,
                ,,,,,,,,,,,,,,#...............@,,,.,@,,,,,,,,,,,,,
                ,,,,,,,,,,,,,@..................&,,,@,,,,,,,,,,,,,
                ,,,,,,,,,,,..................@......@,,,,,,,,,,,,,
                ,,,,,,,,,,,..................@....*&..@,,,,,,,,,,,
                ,,,,,,,,,,@....,............,@......@.,#,,,,,,,,,,
                ,,,,,,,,,,@*.,,.,..*.**.**.,,&....,..,@,,,,,,,,,,,
                ,,,,,,,,,,,,,.,,.*,.,,.,,.,.*,,*,...@@,,,,,,,,,,,,
                ,,,,,,,,,,,,,@*,.*,,*,,*,.,*,,,,,@,,,,,,,,,,,,,,,,
                ,,,,,,,,,,,,,,#,,,,,,,,*,,,,,,,,*,,,,,,,,,,,,,,,,,
                ,,,,,,,,,,,,,,,,@@@,,,,,*@*,,,@,,,,,,,,,,,,,,,,,,,
                ,,,,,,,,,,,,,,,,@*,,,,,,,,,,@@,,,,,,,,,,,,,,,,,,,,
                ,,,,,,,,,,,,@@@@**@,,,,,,,@****@,,,,,,,,,,,,,,,,,,";
    println!("{}",badduck);
}

pub fn dorbell(_: &mut Globals, _ : Args)
{
    println!("cpu when")
}

pub fn astrosam(_: &mut Globals, _: Args)
{
    println!("tf are you doing here go use Z#");
}

pub fn zayther(_: &mut Globals, _ : Args)
{
    println!("1 command = 1 more percent that he'll come back from buying milk")
}

pub fn ovid(globals : &mut Globals,_ : Args)
{
    if globals.graphics.is_inited
    {
        let index = thread_rng().gen_range(0..(globals.graphics.pixels.as_mut().unwrap().get_frame().len())/4-1);
        

        globals.graphics.pixels
                .as_mut()
                .unwrap()
                .get_frame()
                .chunks_exact_mut(4)
                .nth(index)
                .expect("ERR: ovid failed")
                .copy_from_slice(&[0,0,255,255]);
    }
}

pub fn blid(_: &mut Globals, _: Args)
{
    open::that("https://youtu.be/bunrgOHOKMg").unwrap();
}