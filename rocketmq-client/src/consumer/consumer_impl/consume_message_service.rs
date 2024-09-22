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
use std::sync::Arc;

use rocketmq_common::common::message::message_client_ext::MessageClientExt;
use rocketmq_common::common::message::message_ext::MessageExt;
use rocketmq_common::common::message::message_queue::MessageQueue;
use rocketmq_common::ArcRefCellWrapper;
use rocketmq_remoting::protocol::body::consume_message_directly_result::ConsumeMessageDirectlyResult;

use crate::consumer::consumer_impl::pop_process_queue::PopProcessQueue;
use crate::consumer::consumer_impl::process_queue::ProcessQueue;

pub struct ConsumeMessageConcurrentlyServiceGeneral<T, K>
where
    T: ConsumeMessageServiceTrait,
    K: ConsumeMessageServiceTrait,
{
    pub consume_message_concurrently_service: T,
    pub consume_message_pop_concurrently_service: K,
}
pub struct ConsumeMessageOrderlyServiceGeneral<T, K>
where
    T: ConsumeMessageServiceTrait,
    K: ConsumeMessageServiceTrait,
{
    pub consume_message_orderly_service: T,
    pub consume_message_pop_orderly_service: K,
}

pub trait ConsumeMessageServiceTrait {
    fn start(&mut self);

    fn shutdown(&mut self, await_terminate_millis: u64);

    fn update_core_pool_size(&self, core_pool_size: usize);

    fn inc_core_pool_size(&self);

    fn dec_core_pool_size(&self);

    fn get_core_pool_size(&self) -> usize;

    async fn consume_message_directly(
        &self,
        msg: &MessageExt,
        broker_name: &str,
    ) -> ConsumeMessageDirectlyResult;

    async fn submit_consume_request(
        &self,
        msgs: Vec<ArcRefCellWrapper<MessageClientExt>>,
        process_queue: Arc<ProcessQueue>,
        message_queue: MessageQueue,
        dispatch_to_consume: bool,
    );

    async fn submit_pop_consume_request(
        &self,
        msgs: Vec<MessageExt>,
        process_queue: &PopProcessQueue,
        message_queue: &MessageQueue,
    );
}