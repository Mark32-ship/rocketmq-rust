/*
 * Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.
 * The ASF licenses this file to You under the Apache License, Version 2.0
 * (the "License"); you may not use this file except in compliance with
 * the License.  You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use rocketmq_remoting::{
    code::request_code::RequestCode, protocol::remoting_command::RemotingCommand,
    runtime::server::ConnectionHandlerContext,
};
use tracing::info;

#[derive(Default)]
pub struct AdminBrokerProcessor {}

impl AdminBrokerProcessor {
    pub fn process_request(
        &self,
        _ctx: ConnectionHandlerContext,
        request: RemotingCommand,
    ) -> RemotingCommand {
        let request_code = RequestCode::from(request.code());
        info!("AdminBrokerProcessor process_request: {:?}", request_code);
        RemotingCommand::create_response_command()
    }
}
