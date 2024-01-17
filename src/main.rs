
slint::include_modules!();

const TAXPER: f64 = 0.30;
const OWNERPPER: f64 = 0.55;
const PROFITPER: f64 = 0.5;
const OPEXPER: f64 = 0.10;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_divide_income(move | string |{
        let ui:AppWindow = ui_handle.unwrap();
        let num: f64 = string.trim().parse().unwrap();
        let tx: f64 = num*TAXPER;
        let owner: f64 = num*OWNERPPER;
        let profit: f64 = num*PROFITPER;
        let opex: f64 = num*OPEXPER;
        let result: String = format!("Taxes: {:.2}\nOwner: {:.2}\nProfit: {:.2}\nOpEx: {:.2}", {tx}, {owner}, {profit}, {opex});
        ui.set_results(result.into());
    });

    ui.run()
}
