"use client"
import Head from 'next/head'
import Image from 'next/image'
import { Inter } from 'next/font/google'
import { FormEvent, useEffect, useRef, useState } from 'react'
import Link from 'next/link'
import { useRouter } from 'next/router'
import { useSearchParams } from 'next/navigation'

let cnt = 0;
let before = 0
console.log("started", before)

const inter = Inter({ subsets: ['latin'] })
export default function Home() {
  // open = false
  const [dom_v, dom_s] = useState(false);
  const [id_v, id_s] = useState(null);
  const [userName_v, userName_s] = useState(null);
  const [members_v, members_s] = useState(false);
  const {id, un} =  useRouter().query;
  
  const [msg_v, msg_s] = useState([])
  let message = useRef(null);
  const searchParams = useSearchParams()
  const idd = searchParams.get('id')
  
  useEffect( () => {

    // let urlParams = new URLSearchParams(window.location.search);
    // _id = urlParams.get('id');
    id_s(id)
    userName_s(un)
    
    document.addEventListener("mousemove", (e) => {
      dom_s(e.clientX <= 15)
      members_s(window.innerWidth - e.clientX <= 10)
    })
    let f = async (idd) => {
      if (cnt % 80 == 0){
        // console.log("eeeee", window.location.search.match(/id\=(.*)\&/gim))
        console.log("function > idd", idd)
        fetch("http://127.0.0.1:8000/server/info", {
          headers: {
            "name": idd
          }
        }).then(e => { e.json().then(e => {
          document.querySelector("main").innerHTML = e.html;
        }).catch(x => {
          document.querySelector("main").innerHTML = "<h1>ERROR</h1>";
        })}).catch(x => {
          document.querySelector("main").innerHTML = "<h1>ERROR</h1>";
        })
        
      }
      cnt++
      requestAnimationFrame(f)
    }
    console.log("cnt: ", cnt)
    console.log("idd: ", idd)
    console.log("cnt == 0 & idd: ", cnt == 0 & idd != 0)
    if (cnt == 0 & idd != 0) {
      cnt++

      console.log("ok")
      f(idd)
    }
  // if (!open){
  //   open = true
  // }

    
    }, [idd])
      
  const sendMessage = (e) => {
    e?.preventDefault();

    if (message.current?.value){
      fetch("http://127.0.0.1:8000/chat/send", {
        method: "POST",
        headers: {
          "sender": userName_v,
          "content": encodeURI(message.current.value),
          "servername": id_v
        }
      }).then(e => e.json().then(async msg => {
        // let a = Msg(msg)
        // document.querySelector("main").innerHTML = `${a}${document.querySelector("main").innerHTML.replace( "class=\"new\"", "")}`
      }))
      fetch("http://127.0.0.1:8000/server/info", {
          headers: {
            "name": id_v
          }
        }).then(e => { e.json().then(e => {
          document.querySelector("main").innerHTML = e.html;
        })})

      // fetch("http://127.0.0.1:8000/user/create", {
      //   method: "POST",
      //   headers: {
      //     "name": "524",
      //     "email": "yhanbyeol6bg@gmail.com",
      //     "profile_image": "a",
      //   }
      // }).then(e => { console.log(e) })


      fetch("http://127.0.0.1:8000/user/info", {
        method: "GET",
        headers: {
          "name": "a",
        }
      }).then(e => e.json().then(e => {
        console.log("info", e)
      }))



      // fetch("http://127.0.0.1:8000/server/join", {
      //   method: "POST",
      //   headers: {
      //     "servername": "a",
      //     "username": "524",
      //   }
      // }).then(e => { console.log(e) })

      // fetch("http://127.0.0.1:8000/server/create", {
      //   method: "POST",
      //   headers: {
      //     "name": "a"
      //   }
      // }).then( e => { console.log(e) })

      message.current.value = "";
    } else { console.log("non") } 
  }
  return (
    <div id="main">
      <link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Rounded:opsz,wght,FILL,GRAD@48,400,0,0" />

      <nav className={String(dom_v)}>
        <ServerBtn id="logo" select={true}/>
        
        <hr/>
        <ServerBtn id="1"/>
        <ServerBtn id="2"/>
        <ServerBtn id="3"/>
        <ServerBtn id="4"/>
        <ServerBtn id="5"/>
        {userName_v}
        {id_v}
      </nav>

      <main>
      </main>
        
      <form id='messageSender' onSubmit={e => { sendMessage(e) }}>
          <input type="text" ref={message} placeholder='메세지를 입력해주세요' />
          <img src="/icon/send_w.svg" onClick={() => sendMessage()}/>
      </form>

      <aside className={String(members_v)}>
        <UserInfo id='1' name="eaaaaaaaaaaaaaaaa" url="/server/ea.png"/>
        <UserInfo id='1' name="eee" url="ea"/>
        <UserInfo id='1' name="s" url="ea"/>
        <UserInfo id='1' name="ddf" url="ea"/>
        <UserInfo id='1' name="5-23" url="ea"/>
      </aside>
    </div>
  )
}

function ServerBtn(props){
  if (props.id == "logo") {
    return (
      <Link href={`?id=${props.id}`} className={`serverBtn ${props.select}`}>
        <img src={`/logo.svg`}/>
      </Link>
    )
  }
  return (
    <Link href={`?id=${props.id}`} className={`serverBtn ${props.select}`}>
      <img src={`/server/server.svg`}/>
    </Link>
  )
}

function UserInfo(props){
  return (
    <a className={`userInfo`}>
      <img src={`/user/${props.id}.png`}/> <b>{decodeURI(props.name)}</b>
      <span></span>
    </a>
  )
}

// function chatid(id) {
//   var a = {
//   content: null,
//   date: null,
//   sender: null
//   }
//   fetch("http://127.0.0.1:8000/chat/info", {
//     method: "GET",
//     headers: {
//       "chatid": id
//     }
//   }).then(e => e.json().then(e => {
//     a.content = e.content
//     a.date = e.date
//     a.sender = e.sender
//   }))
//   return a
// }

async function chatid(id) {
  const a = await fetch("http://127.0.0.1:8000/chat/info", {
    method: "GET",
    headers: {
      "chatid": id
    }
  })
  return await a.json()
}
