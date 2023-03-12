const express = require('express')
const app = express()
const port = 3000
var cors = require('cors')
const bodyParser = require('body-parser')

app.use(cors())
app.use(bodyParser.json()) // for parsing application/json
app.use(bodyParser.urlencoded({ extended: true })) // for parsing application/x-www-form-urlencoded

app.get('/', (req, res) => {
  res.json('tutorial yew fetch')
})

app.get('/other', (req, res) => {
    res.send("other")
})

app.get('/batman', (req, res) => {
    var user = {
        name: "bruce",
        // superhero: "batman"
    }

    res.send(user)
})

app.post('/bot-homepage', (req, res) => {

  // req.body(data)
  console.log(req.body)
  res.status(200).json('response success')

})

// app.get('/bot-homepage', (req, res) => {

//   res.status(200).send(data).json('response success')
// })

app.post('/attack', (req, res) => {

    console.log(req.body)


  let is_password_correct = true;

  if (is_password_correct) {
    res.status(200).json("attack berhasil")
  } else {
    res.status(400).json("gagal")
  }

})

// app.post('/atlassian.robot.api.dev-domain.site/robots', (res, req) =>{
//   conlose.log(req.body)
//   res.status(200).json('response success')
// })

app.get('/bot-projects', (req, res) => {
  var listuser = [
    {
      name: "Farrel Dava",
      desc: "Bot 1",
      platformEmail: "farrel.davaa@binus.ac.id",
      platformApiKey: "BtwWjdkwjdIWJIE2BN23A23aw4",
      platformType: "Telkom",
      token: "eyHrawwjdhwja12dxZxbSSwdarjnZcnmnjhwekjaANbtkuOOirhj3BnzaAhwu29",
      last_active: 14,
      scheduler: 14,
      checkDoubleName: false,
      checkDoubleEmail: false,
      checkActiveStatus: false,
    },
    {
      name: "Rafael",
      desc: "Bot 2",
      platformEmail: "rafael.tanoe@binus.ac.id",
      platformApiKey: "BtwWjdkwjdIWJIE2BN23A23aw4",
      platformType: "Telkom",
      token: "eyHrawwjdhwja12dxZxbSSwdarjnZcnmnjhwekjaANbtkuOOirhj3BnzaAhwu29",
      active: false,
      last_active: 14,
      scheduler: 14,
      checkDoubleName: false,
      checkDoubleEmail: false,
      checkActiveStatus: false,
    },
    {
      name: "William",
      desc: "Bot 3",
      platformEmail: "william.candra@binus.ac.id",
      platformApiKey: "BtwWjdkwjdIWJIE2BN23A23aw4",
      platformType: "Telkom",
      token: "eyHrawwjdhwja12dxZxbSSwdarjnZcnmnjhwekjaANbtkuOOirhj3BnzaAhwu29",
      last_active: 14,
      scheduler: 14,
      checkDoubleName: false,
      checkDoubleEmail: false,
      checkActiveStatus: false,
    },
    {
      name: "Michael",
      desc: "Bot 4",
      platformEmail: "mic.hael@binus.ac.id",
      platformApiKey: "BtwWjdkwjdIWJIE2BN23A23aw4",
      platformType: "Telkom",
      token: "eyHrawwjdhwja12dxZxbSSwdarjnZcnmnjhwekjaANbtkuOOirhj3BnzaAhwu29",
      last_active: 14,
      scheduler: 14,
      lastActive: 14,
      checkDoubleName: false,
      checkDoubleEmail: false,
      checkActiveStatus: false,
    },
    {
      name: "Gio",
      desc: "Bot 5",
      platformEmail: "gio_kusuma@binus.ac.id",
      platformApiKey: "BtwWjdkwjdIWJIE2BN23A23aw4",
      platformType: "Telkom",
      token: "eyHrawwjdhwja12dxZxbSSwdarjnZcnmnjhwekjaANbtkuOOirhj3BnzaAhwu29",
      last_active: 14,
      scheduler: 14,
      lastActive: 14,
      checkDoubleName: false,
      checkDoubleEmail: false,
      checkActiveStatus: false,
    },
  ];


  let condition = "server error not";

  if (condition == "server error") {
    let error = {
      error_description: "server currently error"
    }
    res.status(500).send(error)
  } else if (condition == "authentication failed") {
    let error = {
      error_description: "auth failed"
    }
    res.status(400).send(error)
  } else {
    let data = {
      list: listuser,
      world: "DC"
    };
    res.status(200).send(data)
  }
})


app.listen(port, () => {
  console.log(`Example app listening on port ${port}`)
})