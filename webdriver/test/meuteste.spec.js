const { By, Builder, Browser } = require('selenium-webdriver');
const { suite } = require('selenium-webdriver/testing');
const assert = require("assert");

suite(function (env) {
  describe('Meu teste', function () {
    this.timeout(30000)
    let driver
    let vars
    beforeEach(async function () {
      driver = await new Builder().forBrowser('chrome').build()
      vars = {}
    })
    afterEach(async function () {
      await driver.quit();
    })
    it('Meu teste', async function () {
      await driver.get("http://localhost:1420/")
      await driver.manage().window().setRect({ width: 1920, height: 1048 })
      await driver.findElement(By.linkText("Tecidos")).click()
      await driver.findElement(By.linkText("Sobre")).click()
      await driver.findElement(By.linkText("Tecidos")).click()
      await driver.findElement(By.linkText("Novo")).click()
      await driver.findElement(By.css(".field:nth-child(1) .input")).click()
      await driver.findElement(By.css(".field:nth-child(1) .input")).sendKeys("Teste")
      await driver.findElement(By.css(".field:nth-child(2) .input")).click()
      await driver.findElement(By.css(".field:nth-child(2) .input")).sendKeys("Fabricante")
      {
        const element = await driver.findElement(By.css(".modal-background"))
        await driver.actions({ bridge: true }).moveToElement(element).clickAndHold().perform()
      }
      {
        const element = await driver.findElement(By.css(".modal-background"))
        await driver.actions({ bridge: true }).moveToElement(element).perform()
      }
      {
        const element = await driver.findElement(By.css(".modal-background"))
        await driver.actions({ bridge: true }).moveToElement(element).release().perform()
      }
      await driver.findElement(By.css(".modal")).click()
      await driver.findElement(By.css(".field:nth-child(3) .input")).sendKeys("1800")
      await driver.findElement(By.css(".field:nth-child(4) .input")).click()
      await driver.findElement(By.css(".field:nth-child(4) .input")).sendKeys("uhvuererf")
      await driver.findElement(By.css(".is-success")).click()
      await driver.findElement(By.css(".is-success")).click()
      await driver.findElement(By.linkText("Cancelar")).click()
      await driver.findElement(By.linkText("Tecidos")).click()
      assert(await driver.findElement(By.linkText("Novo")).getText() == "Novo")
      {
        const element = await driver.findElement(By.linkText("Novo"))
        const locator = `option[@value='${await element.getAttribute("value")}']`
        const selectedText = await element.findElement(By.xpath(locator)).getText()
        assert(selectedText == "Novo")
      }
    })
  })
}, { browsers: [Browser.CHROME, Browser.FIREFOX] });
