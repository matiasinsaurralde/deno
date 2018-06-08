// Copyright 2018 Ryan Dahl <ry@tinyclouds.org>
// All rights reserved. MIT License.
package deno

import (
	"fmt"
	"io/ioutil"
	"net/http"
	"sync"
	"sync/atomic"

	"github.com/golang/protobuf/proto"
)

const (
	httpChan     = "http"
	serverHeader = "Deno"
)

var (
	requestDispatcher *RequestDispatcher
	httpServers       = make(map[int32]*http.Server)
)

// RequestDispatcher takes care of all request-response related commands.
type RequestDispatcher struct {
	// reqs  map[int32]*Request
	reqs  *sync.Map
	reqID *int32
}

// Store constructs an internal request object and stores it into the requests map, returns a pointer to it.
func (d *RequestDispatcher) Store(req *http.Request, res http.ResponseWriter) *Request {
	atomic.AddInt32(d.reqID, 1)
	id := *d.reqID
	r := &Request{
		req:  req,
		res:  res,
		done: make(chan bool),
		id:   id,
	}
	d.reqs.Store(id, r)
	return r
}

// Dispatch will take care of all HTTP_RES_* commands.
func (d *RequestDispatcher) Dispatch(msg *Msg) bool {
	reqPtr, ok := d.reqs.Load(msg.HttpReqId)
	// TODO: handle non-ok here:
	if !ok {
		return ok
	}
	req := reqPtr.(*Request)
	switch msg.Command {
	case Msg_HTTP_RES_WRITE:
		req.res.Write(msg.HttpResBody)
	case Msg_HTTP_RES_STATUS:
		req.res.WriteHeader(int(msg.HttpResCode))
	case Msg_HTTP_RES_END:
		go func() {
			req.done <- true
		}()
	}
	return true
}

// Request wraps http.Request and http.ResponseWriter with additional helper methods.
type Request struct {
	req *http.Request
	res http.ResponseWriter
	id  int32

	// This is used to signal "req.end()":
	done chan bool
}

// ToMsg is a helper method that builds up the request PB object.
func (r *Request) ToMsg() (msg *Msg) {
	msg = &Msg{
		Command: Msg_HTTP_REQ,
		// HttpServerId:  serverID,
		HttpReqPath:   r.req.URL.Path,
		HttpReqMethod: r.req.Method,
		HttpReqId:     r.id,
	}
	return msg
}

func InitHTTP() {
	// Initialize the request dispatcher:
	// var n int32 = 1
	requestDispatcher = &RequestDispatcher{
		reqs:  &sync.Map{},
		reqID: new(int32),
	}
	Sub(httpChan, func(buf []byte) []byte {
		msg := &Msg{}
		check(proto.Unmarshal(buf, msg))
		// If the request ID is present use the request dispatcher:
		if msg.HttpReqId > 0 {
			requestDispatcher.Dispatch(msg)
			return buf
		}

		switch msg.Command {
		case Msg_HTTP_CREATE:
			httpServers[msg.HttpServerId] = &http.Server{}
		case Msg_HTTP_LISTEN:
			httpListen(msg.HttpServerId, msg.HttpListenPort)
		default:
			panic("[http] Unexpected message " + string(buf))
		}
		return buf
	})
}

func buildHTTPHandler(serverID int32) func(w http.ResponseWriter, r *http.Request) {
	return func(w http.ResponseWriter, r *http.Request) {
		req := requestDispatcher.Store(r, w)
		var body []byte
		if r.Body != nil {
			body, _ = ioutil.ReadAll(r.Body)
		}
		msg := req.ToMsg()
		msg.HttpReqBody = body
		go PubMsg(httpChan, msg)

		w.Header().Set("Server", serverHeader)

		// Wait for end signal:
		<-req.done
	}
}

func httpListen(serverID int32, port int32) {
	s := httpServers[serverID]
	listenAddr := fmt.Sprintf(":%d", port)
	handler := buildHTTPHandler(serverID)
	s.Addr = listenAddr
	s.Handler = http.HandlerFunc(handler)
	go s.ListenAndServe()

}
