use std::string::String; 




fn accept_mission_fun() -> () {

    let mut accept_mission: String = String::new();
     std::io::stdin().read_line(&mut accept_mission).expect("Error: Failed to read input (subj: Acceptmission)");
     let accept_mission_uppercase = accept_mission.trim().to_uppercase();

    if accept_mission_uppercase == "YES" {
        println!("\n\nExcellent! Here is a log containing the location of the four required infinity stones: ");
    } //end if 

    else if accept_mission_uppercase == "NO" {
        println!("\n\nOkay then, Tough crowd. It's just the fate of all mankind who cares right.");
        println!("Ending program in 5....4.....3.....2.....1.....");
        std::process::exit(0); //Exit program 
    } //end if 

    else {
        println!("Please just answer with yes or no. \n\n");
        accept_mission_fun(); 
    }; //end if

} //end fn accept_mission




fn happy_ending() -> () {

    println!("\n\n Congratulations! You did it! \n\n");
    println!("After securing all the stones and surviving all the challenges, you find yourself standing at the edge of reality. With the stones in your possession, you have the power to rewrite reality. In a breath-taking moment, you snap your fingers, and a flash of light ripples across the multiverse.");
    println!("Opening your eyes, you find yourself back on Earth, with your companion by your side. There's no sign of Hitler Jr. or his forces. The stones have once again scattered across the multiverse, and peace has been restored. You have changed the course of history and saved countless worlds from tyranny. You and your companion are celebrated as heroes, the ones who made the impossible possible.");
    std::process::exit(0); //exit program 


}




struct Choices {

    already_picked_a: bool,
    already_picked_b: bool,
    already_picked_c: bool,
    already_picked_d: bool,
}




fn select_location(choices_made: &mut i8, choices: &mut Choices) -> () {

    if *choices_made == 4 { 
        
        happy_ending(); return;
    } //end if 


    let mut location_letter: String = String::new();
    std::io::stdin().read_line(&mut location_letter).expect("Error: Failed to read input (subj: location letter)");
    let location_letter_upper:char = location_letter.trim().to_uppercase().chars().next().unwrap_or(' ');
    
    if location_letter_upper == 'A' && choices.already_picked_a == false{
        
        choices.already_picked_a = true;
        *choices_made += 1;
        println!("Understood, Trasporting to dimension 24D in 5....4....3....2....1....");
        power_stone_journey(choices_made, choices);
    } //end if 
    
    else if location_letter_upper == 'B' && choices.already_picked_b == false{

        choices.already_picked_b = true;
        *choices_made += 1;
        println!("Understood, Trasporting to dimension 53C in 5....4....3....2....1....");
        time_stone_journey(choices_made, choices);
    } //end else if 

    else if location_letter_upper == 'C' && choices.already_picked_c == false{

        choices.already_picked_c = true;
        *choices_made += 1;
        println!("Understood, Trasporting to dimension 22P in 5....4....3....2....1....");
        reality_stone_journey(choices_made, choices);
    } //end elseif 

    else if location_letter_upper == 'D' && choices.already_picked_d == false{

        choices.already_picked_d = true; 
        *choices_made += 1;
        println!("Understood, Trasporting to dimension 66Z in 5....4....3....2....1....");
        soul_stone_journey(choices_made, choices);
    } //end else if 

    else {

        println!("Invalid input: please enter A, B, C or D, If you have already picked one it cannot be chosen again.");
        select_location(choices_made, choices);
} //end else 

} //end fn select_location()




fn power_stone_journey(choices_made: &mut i8, choices: &mut Choices) -> () {


    println!("Welcome to Dimension 24D. In a reality where World War II never ended, the Pentagon is an impenetrable fortress. As you stand, dwarfed by its imposing structure, you quickly realize this won't be easy. Your mission: to penetrate the fortress and secure the Power Stone.

    With your companion, you start infiltrating the base. At your first checkpoint, you face an advanced security system, an intricate combination of numbers, lasers, and sensors. Here, you're presented with a complex math problem, a pattern of numbers which is the key to deactivating the security grid. Failure to solve it would trigger an alarm, alerting the guards to your presence."); 

    //Math question here

    println!("Once past the security, you descend into the depths of the Pentagon. The corridors are patrolled by heavily armed guards. Here you face your second checkpoint, a fight against time as you sneak past the guards, calculating their patrol patterns and timing your movements perfectly. If you're spotted, it's game over.");


    //Math question here 

    println!("Finally, you arrive at the vault. Inside, the Power Stone is housed, pulsating with raw, destructive energy. But there's one final obstacle - the vault's lock, which can only be opened by solving an array of mathematical problems. A wrong answer would trigger a self-destruction protocol, resulting in a certain game over.");


    //Math question here

    println!("Please select your next destination, or if you have already succesfully completed all 4 destinations simply press enter");

    select_location(choices_made, choices); 

}




fn time_stone_journey(choices_made: &mut i8, choices: &mut Choices) -> () {

    println!("Dimension 53C welcomes you with the sun glinting off grand pyramids. In this reality, Ancient Egypt never fell, and the Time Stone is hidden deep within the largest pyramid. Your mission is to navigate the labyrinth, pacify the ancient guardians, and secure the stone.

    As you venture into the pyramid, the first checkpoint awaits. The hallways are filled with deadly traps that work on timers. To disable them, you must solve math problems relating to time and sequence. Failing to answer correctly would spring the trap, ending your journey instantly."); 

    //Math question here

    println!("Venturing deeper, time warps around you, creating illusions and reality shifts. Here's your second checkpoint. You must calculate and predict these shifts to avoid being lost in time forever. A single miscalculation would leave you trapped in an endless loop.");


    //Math question here 

    println!("At the heart of the pyramid, guarded by the time-warped Pharaohs, is the Time Stone. To pacify the Pharaohs and retrieve the stone, you must solve their ancient riddles, a series of math problems based on the Egyptian numeral system. Failure would result in being trapped in the pyramid for eternity.");


    //Math question here

    println!("Please select your next destination, or if you have already succesfully completed all 4 destinations simply press enter");

    select_location(choices_made, choices); 
    
}




fn reality_stone_journey(choices_made: &mut i8,choices: &mut Choices ) -> () {

    println!("Dimension 22P is a world out of a storybook, where every myth, legend, and fairy tale is real. You're tasked to find the Reality Stone, hidden within a grand, ever-shifting museum.

    As you step inside, reality begins to shift around you. Buildings float, rivers flow in the sky, and animals talk. Your first checkpoint involves navigating this ever-changing landscape by predicting the shifts and finding the path to the stone. Failure to calculate correctly would result in getting lost in this reality forever."); 

    //Math question here

    println!("As you make your way through the museum, you encounter a series of magical guardians. At this second checkpoint, you must solve math riddles that these guardians pose. Every wrong answer would change your reality, with failure turning you into a statue guarding the museum.");


    //Math question here 

    println!("At the heart of the museum, the Reality Stone awaits, but it's protected by a powerful enchantment that can only be broken by solving a series of intricate math problems. Incorrect answers would alter your reality, turning you into a part of the museum's exhibitions forever.");


    //Math question here

    println!("Please select your next destination, or if you have already succesfully completed all 4 destinations simply press enter");

    select_location(choices_made, choices); 
    
}




fn soul_stone_journey(choices_made: &mut i8, choices: &mut Choices) -> () {

    println!("Dimension 66Z is a grim, desolate world overrun by Hitler Jr.'s forces. The location of the Soul Stone is unknown, but it's said to be protected by a sacrifice of something you love. Your task is to uncover the stone and escape this forsaken place."); 
    println!("As you delve into the ruins of this world, your first checkpoint is a battalion of Hitler Jr.'s forces. You must calculate their positions and plan a strategy to evade them. Any miscalculations would result in capture and immediate game over.");

    //Math question here

    println!("As you push further, you find a mystical shrine that responds to heartfelt confessions. Here, at the second checkpoint, you must solve a set of emotional math problems, dealing with relationships and values. Incorrect answers would result in the shrine denying you the location of the stone.");


    //Math question here 

    println!("Once the shrine reveals the Soul Stone's location, you face the final challenge. You must calculate the cost of the necessary sacrifice and make the hard decision. A wrong calculation would result in a chain reaction, costing you everything, including your life.");


    //Math question here

    println!("Please select your next destination, or if you have already succesfully completed all 4 destinations simply press enter");

    select_location(choices_made, choices); 
    
}




fn main() {

    let mut nickname: String = String::new();    
    let mut companion_species: String = String::new(); 
    let mut companion_name: String = String::new(); 
    let mut choices_made: i8 = 0;
    let mut choices = Choices {
        already_picked_a: false,
        already_picked_b: false,
        already_picked_c: false,
        already_picked_d: false,
    };

    println!("\n\nHello, world! and welcome!\n\nToday we combine math with adventure. In this interactive simulation you and the companion of your choice will be transported on a mission to fight villains and save the world from it's impending doom. Here is how it will work.\n");
    println!("..................................................................................................................................................................................................................................................................................");
    println!("\nOn your mission you will encounter a series of obstacles. When you encounter such obstacles you will be given a math question, answer all the questions correct and you are sure to succeed thus saving the world and claiming fame and glory in the process; however, answer incorrect and your story will come to a sudden and not so pleasent ending.\n\n");
    println!("To begin Enter you chosen nickname, and the companion of your choice. NOTE: Your chosen companion can be any animal of your choice\n\n");
    println!("Nickname: \n"); 
    std::io::stdin().read_line(&mut nickname).expect("Error: Failed to read input (subj: Nickname)"); 
    println!("Companion's species: \n");
    std::io::stdin().read_line(&mut companion_species).expect("Error: Failed to read input (subj: Companion.species)"); 
    println!("Companion's Name: \n");
    std::io::stdin().read_line(&mut companion_name).expect("Error: Failed to read input (subj: Companion.name");
    println!("\n\n Excellent! It's a pleasure to meet you {} as well your your companion {} the great {} \n\n", nickname, companion_name, companion_species);
    println!("Now then let us begin......");
    println!("\n\n{}, as you are already aware the world as we know it is in terrible danger! As the final surviving members of the The Nexus initiative task force it is all up to you and your companion {} the great {} .", nickname, companion_name, companion_species);
    println!("..................................................................................................................................................................................................................................................................................");
    println!("\n\nMission Debrif: Operation Codename: Final Stand....................");
    println!("\n\nTime is of the essence. The fate of our reality - and countless others - hangs in the balance. An interdimensional traveler who we have come to know as Hitler Jr., wielding technology far superior to ours and fortified by the military strength of a multiversal empire, has set his sights on our dimension. Our analysis predicts an imminent full-scale invasion. If we do not act, our world will fall, just like the countless others before us. Our combat capabilities are significantly inferior to Hitler Jr.'s forces. An open confrontation would be catastrophic and result in the enslavement of all humankind. Therefore, we have devised an unconventional strategy that centers on the four Infinity Stones, cosmic artifacts of immense power. Our intelligence indicates that these stones are scattered across four separate dimensions. Your mission, should you choose to accept, will be to travel to these dimensions and secure all four Infinity Stones. Each Stone represents a facet of existence itself - Power, Time, Reality and Soul. United, they wield enough energy to reshape reality itself.\n\n Your mission is fraught with danger. You and your companion will face hostile environments, powerful adversaries, and the ever-looming threat of Hitler Jr.'s forces. But this is a mission of paramount importance - the only viable course of action we have left. With the combined power of the Infinity Stones, you will have the capability to eliminate Hitler Jr. and his army from existence - a snap, a flash, and our universe would be saved. Remember, while this mission is critical, it is equally important to preserve the balance of the multiverse. The power of the Infinity Stones is not to be taken lightly. Once used the stones will once again be scattered at random across the multiverse so you will only get one shot at this. You are our final hope, our last stand against a tyrant set on universal domination. Your mission is clear, your purpose unwavering. The survival of our world, and countless others, rests on your shoulders.\n\n\n");
    println!("Do you accept this mission?");
    accept_mission_fun();
    println!(" Location A: The Power stone is located in demension 24D burried below the pentagon\n Location B: The Time stone is located in demension 53C within an Egyptian pyramid\n Location C: The Reality stone is located in demension 22P in a meusam\n Location D: The Soul stone is located in demension 66Z a demension belived to already be overrun by Hitler jr, the stones exact location is unknnown.");
    println!("\n\n Please Enter the letter of the location you would first like to start your journey: ");
    select_location(&mut choices_made, &mut choices); 




} //end main() fn
