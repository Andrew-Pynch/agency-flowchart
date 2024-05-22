use std::io::{self};

fn main() {
    println!("How to increase your agency: a flowchart");
    println!(
        "Agency: accepting the world, nticing paths to your goals, noticing what your goals are"
    );

    println!("You have a problem: are you working on it? (y/n)");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let working_on_it = input.trim().to_lowercase();

    if working_on_it == "n" {
        println!("Are you in a victim mindset? (y/n)");
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let victim_mindset = input.trim().to_lowercase();

        if victim_mindset == "y" {
            println!("Can you answer these questions? (y/n)");
            println!("- What if it were possible?");
            println!("- What's the stupidest, easiest thing you could do to make even a little bit of progress?");
            println!("- Why are you so sure you won't succeed?");
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let can_answer = input.trim().to_lowercase();

            if can_answer == "n" {
                println!(
                    "Try to reflect on these questions and see if you can shift your mindset."
                );
            } else {
                println!("Great! Answering these questions can help you break free from the victim mindset.");
            }
        } else {
            println!("Can you answer these questions?");
            println!("(nw) What are you doing right now?");
            println!("(goal) What is a detailed description of the world after you've succeeded? What does success look like? What are you actually trying to do? In 12 months, what would you like to be celebrating with a friend?");
            println!("(why) Why do you want to do that?");
            println!("(problem) What is the roadblock? What is the problem in detail? What might make you procrastinate?");
            println!("(solutions) What are some ideas that could possibly work?");
            println!("(plan) Given the best, easiest, or most liked idea, what's your rough draft of a plan?");
            println!("(next step) What's the immediate next step?");
            println!("(help) Who or what could help fill in the gaps?");
            println!("(environment) How could you set up your social context and environment to bolster your motivation, and avoid frustrations and temptations?");
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let can_answer = input.trim().to_lowercase();

            if can_answer == "y" {
                println!("Great! Answering these questions can help you clarify your goals and take action.");
            } else {
                println!("Try to reflect on these questions and see if you can find clarity and motivation.");
            }
        }
    } else {
        println!("Can you answer these questions? (y/n)");
        println!("- What is a detailed description of the world after you've succeeded?");
        println!("- What's the connection between what you're doing nw, your plan, and the goal?");
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let can_answer = input.trim().to_lowercase();

        if can_answer == "y" {
            println!("Remember:");
            println!("People tend to:");
            println!("- Underestimate the likelihood of success for \"bad\" plans that could work");
            println!("- Overestimate the likelihood of success for sure-thing plans (day-to-day is usually more chaotic than we expect)");
            println!("Free yourself from the requirement that your ideas must be good. All that matters is that they're possible. It could work.");
        } else {
            println!("Try to reflect on these questions and see if you can find clarity and alignment between your actions and goals.");
        }
    }
}
