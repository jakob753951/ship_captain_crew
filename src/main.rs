use rand::Rng;

#[derive(Debug)]
struct Fleet {
    ship: Option<Ship>,
    captain: Option<Captain>,
    crew: Option<Crew>,
    cargo: u8,
    remaining_dice: u8,
}

impl Fleet {
    fn new() -> Self {
        Self {
            ship: None,
            captain: None,
            crew: None,
            cargo: 0,
            remaining_dice: 5,
        }
    }

    fn is_complete(&self) -> bool {
        self.ship.is_some() && self.captain.is_some() && self.crew.is_some()
    }

    fn add_ship(&mut self) -> Result<(), ()> {
        if self.ship.is_some() {
            return Err(());
        }

        self.ship = Some(Ship);
        Ok(())
    }

    fn add_captain(&mut self) -> Result<(), ()> {
        if self.ship.is_none() {
            return Err(());
        }

        if self.captain.is_some() {
            return Err(());
        }

        self.captain = Some(Captain);
        Ok(())
    }

    fn add_crew(&mut self) -> Result<(), ()> {
        if self.captain.is_none() {
            return Err(());
        }

        if self.crew.is_some() {
            return Err(());
        }

        self.crew = Some(Crew);
        Ok(())
    }
}

fn roll(fleet: &mut Fleet) {
    let mut rolled_dice = (0..fleet.remaining_dice)
        .map(|_| rand::thread_rng().gen_range(1..=6))
        .collect::<Vec<u8>>();
    
    println!("Rolled: {:?}", rolled_dice);

    if rolled_dice.contains(&6) && fleet.add_ship().is_ok() {
        rolled_dice.remove(rolled_dice.iter().position(|&x| x == 6).unwrap());
    }

    if rolled_dice.contains(&5) && fleet.add_captain().is_ok() {
        rolled_dice.remove(rolled_dice.iter().position(|&x| x == 5).unwrap());
    }

    if rolled_dice.contains(&4) && fleet.add_crew().is_ok() {
        rolled_dice.remove(rolled_dice.iter().position(|&x| x == 4).unwrap());
    }

    if fleet.is_complete() {
        fleet.cargo = rolled_dice.iter().sum();
    }

    fleet.remaining_dice = rolled_dice.len() as u8;
}

#[derive(Debug)]
struct Ship;
#[derive(Debug)]
struct Captain;
#[derive(Debug)]
struct Crew;

fn main() {
    let fleet = &mut Fleet::new();

    println!("Starting fleet: {:?}", fleet);
    
    loop {
        println!("Press enter to roll, or type q and enter to quit");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        
        if input.trim() == "q" {
            break;
        }
        
        roll(fleet);
        println!("Fleet: {:#?}", fleet);
    }
}
