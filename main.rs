use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiContextPass};

fn main() {
    App::new()
    .add_plugins(
        DefaultPlugins

            .set(WindowPlugin {
                primary_window: Some(Window {
                    // You may want this set to `true` if you need virtual keyboard work in mobile browsers.
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }),
    )
            .add_plugins(EguiPlugin { enable_multipass_for_primary_context: true })
        .add_systems(EguiContextPass, ui_example_system)
        .add_systems(Startup, startup)

        .run();
}

fn startup (mut commands: Commands) {

    commands.spawn(Ya {yy: r###"
summon minecraft:allay
summon minecraft:allay
/setblock ~ ~ ~3 minecraft:command_block[conditional=false,facing=north]{Command:"summon minecraft:armor_stand",LastExecution:3235672L,LastOutput:'{"text":"[00:02:35] ","extra":[{"translate":"commands.summon.success","with":[{"translate":"entity.minecraft.armor_stand","hoverEvent":{"contents":{"type":"minecraft:armor_stand","id":[-1151219553,2121876774,-1206100483,2091083959],"name":{"translate":"entity.minecraft.armor_stand"}},"action":"show_entity"},"insertion":"bb61c89f-7e79-4526-b81c-5dfd7ca368b7"}]}]}',SuccessCount:1,TrackOutput:1b,UpdateLastExecution:1b,auto:1b,conditionMet:1b,powered:0b}"###.to_string(),ag: r###""###.to_string(),bg: r###""###.to_string(),cg: r###""###.to_string(),yye: r###""###.to_string(),yye1: r###""###.to_string(),yye3: r###""###.to_string(),yye2: r###""###.to_string(),num_rows: 250.000,row_height: 250.000});
}

#[derive(Component)]
struct Ya {
    yy: String,
    yye: String,
    yye1: String,
    yye2: String,
    yye3: String,


    ag: String,
    bg: String,
    cg: String,
    num_rows: f32,
    row_height: f32,
}

fn ui_example_system(mut commands: Commands,mut my_string: Query<&mut Ya>,mut contexts: EguiContexts) {
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {

        
for mut aa in my_string {



    egui::ScrollArea::vertical()
    .auto_shrink([false; 2])
    .show_viewport(ui, |ui, viewport| {
        ui.set_height(aa.row_height * aa.num_rows as f32);






    ui.label("rust code by ComputersAreCool https://www.youtube.com/@computersRcool               https://www.reddit.com/user/ComputersAreC/");
    ui.add_space(5.0);
    ui.label("thank you to u/TahoeBennie for creating the one command format https://www.reddit.com/user/TahoeBennie/ ");
    ui.add_space(25.0);


    ui.label("tutorial here");
    ui.label("https://www.youtube.com/watch?v=yqO8nl1krX4");


    #[derive(PartialEq)]
enum Enum { First, Second,  }
ui.horizontal(|ui| {
    ui.add_sized(egui::vec2(40.0, 10.0), egui::TextEdit::singleline(&mut aa.yye1));
    ui.add_sized(egui::vec2(40.0, 10.0), egui::TextEdit::singleline(&mut aa.yye2));
    ui.add_sized(egui::vec2(40.0, 10.0), egui::TextEdit::singleline(&mut aa.yye3));

});

let mut gg1 = aa.yye1.parse::<f32>();

let mut gg1 = match gg1 {
    Ok(file) => file,
    Err(error) => return {ui.label("put in relative coordinates");},
};

let mut gg2 = aa.yye2.parse::<f32>();

let mut gg2 = match gg2 {
    Ok(file) => file,
    Err(error) => return {ui.label("put in relative coordinates");},
};

let mut gg3 = aa.yye3.parse::<f32>();


let mut gg3 = match gg3 {
    Ok(file) => file,
    Err(error) => return {ui.label("put in relative coordinates");},
};
        

    ui.text_edit_multiline(&mut aa.yy);
    if ui.button("Click me to generate 1 command 1.21.5+").clicked() {
        
        let mut a = String::from(r#"summon falling_block ~ ~.8 ~ {BlockState:{Name:redstone_block},Passengers:[{id:falling_block,BlockState:{Name:activator_rail}},{id:command_block_minecart,Command:"gamerule commandBlockOutput false"},{id:command_block_minecart,Command:"setblock ~ ~-2 ~ repeating_command_block{auto:1,Command:'fill ~ ~ ~ ~ ~2 ~ air'}"},{id:command_block_minecart,Command:""#);
 
        let mut b= String::from("");

        b.push_str(&mut aa.yy.clone());

        let mut ee: u32;
        let mut dde= String::from("");
        let mut dde2= String::from("");
        let mut dde3= String::from("");
        let mut eaa = 0.;
        let mut eaa2 = 0.;
        let mut eaa3 = 0.;
        let mut ooo3 = false; 
        let mut ooo2 = false; 
        let mut ooo1 = false; 
        let mut hh1 = false; 
        let mut fffe = 0; 

        let mut aaa = b.len();
        for mut aa in 0..aaa {

            fffe = 0;
            hh1 = false;
            if aa == 0 {
                

                hh1 = true;

                fffe = 1;

            }

            if b.chars().nth(aa) == Some('\n') {hh1 = true}

            if hh1 == true {
                loop {
                    if b.chars().nth(aa + 1 - fffe) == Some(' ') {let mut b = b.remove(aa + 1 - fffe);} else {
                        if b.chars().nth(aa + 1 - fffe) == Some('/') {let mut b = b.remove(aa + 1 - fffe);}; 
                        if b.chars().nth(aa + 1 - fffe) == Some('s') {
                            if b.chars().nth(aa + 2 - fffe) == Some('e') {
                                if b.chars().nth(aa + 3 - fffe) == Some('t') {
                                    if b.chars().nth(aa + 4 - fffe) == Some('b') {
                                        if b.chars().nth(aa + 5 - fffe) == Some('l') {
                                            if b.chars().nth(aa + 6 - fffe) == Some('o') {
                                                if b.chars().nth(aa + 7 - fffe) == Some('c') {
                                                    if b.chars().nth(aa + 8 - fffe) == Some('k') {
                                                        if b.chars().nth(aa + 9 - fffe) == Some(' ') {
                                                            

                                                                let mut dde= String::from("");
                                                                let mut dde2= String::from("");
                                                                let mut dde3= String::from("");
                                                                
                                                                let odddd: bool;

                                                                'sss: loop {
                                                                    if b.chars().nth(aa + 10 - fffe) == Some(' ') {
                                                                        odddd = false;break;
                                                                    } else {
                                                                        if b.chars().nth(aa + 10 - fffe) == Some('~') {
                                                                            odddd = true; loop {
                                                                                if b.chars().nth(aa + 10 - fffe) == Some(' ') {
                                                                                    break 'sss;
                                                                                }
                                                                                aa = aa + 1;
                                                                                
                                                                            }
                                                                        } 
                                                                        dde.push(b.chars().nth(aa + 10 - fffe).unwrap());
    
                                                                        
                                                                        let mut b = b.remove(aa + 10 - fffe);
                                                                    }
                                                                }
                                                                
                                                                if odddd == false {let mut dde = dde.parse::<f32>();
                                                                    
                                                                    let mut dde = match dde {
                                                                        Ok(file) => file,
                                                                        Err(error) => return println!("33error"),
                                                                    };
                                                                
                                                                
                                                                    if ooo1 == false {
                                                                        eaa = dde;
                                                                        ooo1 = true;
                                                                        eaa = eaa - gg1

                                                                    }

                                                                    dde = dde - eaa;

                                                                    let mut dde = dde.to_string();

                                                                    b.insert_str(aa + 10 - fffe, "~");
                                                                    aa = aa + 1;
                                                                    b.insert_str(aa + 10 - fffe, &dde);
                                                                    aa = aa + dde.len();


                                                                }

                                                                let odddd2: bool;
                                         
                                                                's: loop {
                                                                    if b.chars().nth(aa + 11 - fffe) == Some(' ') {
                                                                        odddd2 = false;break;
                                                                    } else {
                                                                        if b.chars().nth(aa + 11 - fffe) == Some('~') {
                                                                            odddd2 = true; loop {
                                                                                if b.chars().nth(aa + 11 - fffe) == Some(' ') {
                                                                                    break 's;
                                                                                }
                                                                                aa = aa + 1;
                                                                                
                                                                            }
                                                                        } 
                                                                        dde2.push(b.chars().nth(aa + 11 - fffe).unwrap());
    
                                                                        
                                                                        let mut b = b.remove(aa + 11 - fffe);
                                                                    }
                                                                }


                                                                if odddd2 == false {let mut dde2 = dde2.parse::<f32>();
                                                                    
                                                                    let mut dde2 = match dde2 {
                                                                        Ok(file) => file,
                                                                        Err(error) => return println!("22error"),
                                                                    };
                                                                
                                                                                                                                
                                                                    if ooo2 == false {
                                                                        eaa2 = dde2;
                                                                        ooo2 = true;
                                                                        eaa2 = eaa2 - gg2

                                                                    }

                                                                    dde2 = dde2 - eaa2;

                                                                    println!("{dde2}");

                                                                    let mut dde2 = dde2.to_string();

                                                                    b.insert_str(aa + 11 - fffe, "~");
                                                                    aa = aa + 1;

                                                                    b.insert_str(aa + 11 - fffe, &dde2);
                                                                    aa = aa + dde2.len();


                                                                }
                                                                
                                                                let odddd3: bool;
                                                                'ss: loop {
                                                                    if b.chars().nth(aa + 12 - fffe) == Some(' ') {
                                                                        odddd3 = false;break;
                                                                    } else {
                                                                        if b.chars().nth(aa + 12 - fffe) == Some('~') {
                                                                            odddd3 = true; loop {
                                                                                if b.chars().nth(aa + 12 - fffe) == Some(' ') {
                                                                                    break 'ss;
                                                                                }
                                                                                aa = aa + 1;
                                                                                
                                                                            }
                                                                        } 
                                                                        dde3.push(b.chars().nth(aa + 12 - fffe).unwrap());
    
                                                                        
                                                                        let mut b = b.remove(aa + 12 - fffe);
                                                                    }
                                                                }
                                                                println!("{dde3}");

                                                                    if odddd3 == false {let mut dde3 = dde3.parse::<f32>();
                                                                    
                                                                    let mut dde3 = match dde3 {
                                                                        Ok(file) => file,
                                                                        Err(error) => return println!("11error"),
                                                                    };
                                                                
                                                                    if ooo3 == false {

                                                                        eaa3 = dde3;
                                                                        ooo3 = true;
                                                                        eaa3 = eaa3 - gg3
                                                                    }
                                                                    
                                                                    dde3 = dde3 - eaa3;
        
                                                                        println!("{} {} {}",dde, dde2 ,dde3);
                                                                        let mut dde3 = dde3.to_string();

                                                                        b.insert_str(aa + 12 - fffe, "~");
                                                                        aa = aa + 1;
                                                                        b.insert_str(aa + 12 - fffe, &dde3);

                                                                        aa = aa + dde3.len();


                                                                        
                                                                        println!("{b}");
                                                                        
        
                                                                }
                                                                



                                                                println!("{} {} {}",dde, dde2 ,dde3);

                                                            
                                                            

                                                        
                                                        }; 
                                                        
                                                    }; 
                                                }; 
                                            }; 
                                        }; 
                                    }; 
                                }; 
                            }; 
                        }; 

                        
                        
                        break;
                    }
                    

                }

            }
        }
        let mut b = b.trim_start();

        println!("{}",b);
         
         
            let mut b = b.replace(r#"""#, r#"\""#);
         
         
            let mut b = b.replace("\n", r#""},{id:command_block_minecart,Command:""#)
         
            ;
         
         
            let mut c = r#""},{id:command_block_minecart,Command:"execute align xz run kill @e[type=command_block_minecart,dy=0]"}]}"#;
         
         
         
            aa.ag = a.clone();
            aa.ag.push_str(&b.clone());
            aa.ag.push_str(&c.clone().to_string());



    }

    ui.label(format!("{}",aa.ag));
});

}




    });
}
