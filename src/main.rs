use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "Impermanent Loss Calculator", long_about = None)]
struct Args {
    #[arg(short, long)]
    initial_price: f64,

    #[arg(short, long)]
    final_price: f64,
}

fn calculate_il(initial_price: f64, final_price: f64) -> f64 {
    let price_ratio = final_price / initial_price;
    // IL = 2 * sqrt(price_ratio) / (1 + price_ratio) - 1
    let il = (2.0 * price_ratio.sqrt()) / (1.0 + price_ratio) - 1.0;
    il
}

fn main() {
    let args = Args::parse();

    if args.initial_price <= 0.0 || args.final_price <= 0.0 {
        println!("Prices must be greater than zero.");
        return;
    }

    let il_decimal = calculate_il(args.initial_price, args.final_price);
    // Real IL is usually a negative value but percentage is presented as absolute
    let il_percentage = il_decimal.abs() * 100.0;

    println!("Initial Price: ${:.2}", args.initial_price);
    println!("Final Price:   ${:.2}", args.final_price);
    println!("Price Ratio:   {:.2}x", args.final_price / args.initial_price);
    println!("Impermanent Loss: {:.2}%", il_percentage);
    
    let held_value = args.final_price;
    let pool_value = 2.0 * args.initial_price * (args.final_price / args.initial_price).sqrt() / 2.0; 
    // This is a simplification; realistically just use IL % on $100
    println!("If you deposited $100:\n- Held value: ${:.2}\n- Pool value: ${:.2}\n- Loss vs holding: ${:.2}", 
        100.0 * (args.final_price / args.initial_price), 
        100.0 * (args.final_price / args.initial_price) * (1.0 - il_decimal.abs()),
        100.0 * (args.final_price / args.initial_price) * il_decimal.abs()
    );
}
