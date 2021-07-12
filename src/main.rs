use piston_window::*;
use rand::*;

const HEIGHT:f64=1280.0;
const WIDTH:f64=720.0;

struct Bubble{
    speed:f64,x:f64,y:f64,r:f64
}

impl Bubble{
    pub fn new(num:Option<f64>) -> Bubble{//we've num as option as we want some bubble's to start from top/bottom etc
        let r=(random::<f64>()*(WIDTH/8.0))+5.0;
        let mut b:Bubble=Bubble{
            speed:(random::<f64>()*90.0)+10.0,//random float 64 value
            x:random::<f64>()*WIDTH,
            y:random::<f64>()*(HEIGHT+r),
            r:r
        };
        if let Some(y)=num{
            b.speed=0.0;
            b.y=y
        }
        return b
    }//public function "new"
}

fn get_bubbles() -> Vec<Bubble>{//we don't have to put a lifetime(when retunrning a reference) as we're returning actual data
    //rust know's not to kill actual data but could kill reference
    let mut bubbles=Vec::new();
    let n=(random::<u64>()%15)+10;//unsigned integers as we need +ve number of bubbles
    for _ in 0..n{
        bubbles.push(Bubble::new(Some(HEIGHT)));
        bubbles.push(Bubble::new(Some(0.0)));
        bubbles.push(Bubble::new(None));//some random Y coordinate
    }
    bubbles
}

fn main() -> (){
    let bub=[1.0, 97.0/255.0, 0.0, 1.0];//RGBA value b/w 0 & 1
    let bg=[104.0/255.0, 221.0/255.0, 19.0/255.0, 1.0];
    let mut bubbles:Vec<Bubble>=get_bubbles();//bubbles will have the ownership of the data that get_bubbles return 
    let mut window:PistonWindow=WindowSettings::new("Lava Lamp", [WIDTH, HEIGHT])
        .exit_on_esc(true).build().unwrap();
    let mut events=window.events;//grab a particular event
    while let Some(e)=events.next(&mut window){//single event loop in piston
        if let Some(_)=e.render_args(){//is we're going to draw a window
            //draw bubble code here
            let bubbs=&bubbles;
            window.draw_2d(&e,|c,g,_|{
                clear(bg, g);//clear the screen with background colour
                for b in bubbs{
                    ellipse(bub, [b.x-b.r, b.y-b.r, b.r*2.0, b.r*2.0], c.transform, g)//piston function
                                  //x,y,width,height
                }
            });()

        }//if we're getting current events from piston the central event loop is to draw the window
        if let Some(u)=e.update_args(){
            let bubbs=&mut bubbles;//do it without giving ownership to it
            //it is bounded inside of the scope so it will loost ownership and then we derference bubbles
            for b in bubbs{//for every bubble
                b.y-=b.speed*u.dt;//dt gives us the delta time move the bubble up by speed * how many millisec's have passed
                if b.y+b.r<=0.0{b.y=HEIGHT+b.r}//if the bubble go above the top of the screen then we will put it to bottom
            }
        }//if it's to update the simulation we update it here
    }
}