const { By, Builder, Browser } = require('selenium-webdriver');
const { suite } = require('selenium-webdriver/testing');
const assert = require("assert");

suite(function (env) {
    describe('Fabric script', function () {
        let driver;

        before(async function () {
            driver = await new Builder().forBrowser('chrome').build();
        });

        after(async () => await driver.quit());

        it('Testing fabric page', async function () {
            await driver.get('http://localhost:1420/fabric');

            let title = await driver.getTitle();
            assert.equal("Textile V2.1", title);

            await driver.manage().setTimeouts({ implicit: 500 });

            let labelBox = await driver.findElement(By.id('title'));
            let value = await labelBox.getText();
            assert.equal("Tecido", value);
        });
    });
}, { browsers: [Browser.CHROME, Browser.FIREFOX] });