// Copyright (c) RoochNetwork
// SPDX-License-Identifier: Apache-2.0

module rooch_examples::coin_factory {
   use std::string::{Self, String};
   use std::vector;
   use moveos_std::signer;
   use moveos_std::table::{Self, Table};
   use moveos_std::account_storage;
   use moveos_std::context::Context;
   use moveos_std::move_module;

   const TEMPLATE_MODULE_ADDRESS: address = @0xdeadeadeadeadeadeadeadeadeadeadeadeadeadeadeadeadeadeadeadeadead;
   const TEMPLATE_MODULE_IDENTIFIER: vector<u8> = b"coin_module_identifier_placeholder";
   const TEMPLATE_COIN_STRUCT_IDENTIFIER_PLACEHOLDER: vector<u8> = b"COIN_STRUCT_IDENTIFIER_PLACEHOLDER";
   const TEMPLATE_COIN_NAME_PLACEHOLDER: vector<u8> = b"COIN_NAME_PLACEHOLDER";
   const TEMPLATE_COIN_SYMBOL_PLACEHOLDER: vector<u8> = b"COIN_SYMBOL_PLACEHOLDER";
   const TEMPLATE_COIN_SUPPLY_PLACEHOLDER: u256 = 123_321_123_456u256;
   const TEMPLATE_COIN_DECIMALS: u8 = 222u8;

   struct TemplateStore has key, store {
      templates: Table<String, vector<u8>>,
   }

   fun init(ctx: &mut Context) {
      let module_signer = signer::module_signer<TemplateStore>();
      let templates = table::new<String, vector<u8>>(ctx);
      account_storage::global_move_to(ctx, &module_signer, TemplateStore { templates });
   }

   public entry fun register_template(ctx: &mut Context) {
      let name = string::utf8(b"fixed_supply_coin");
      let template_bytes = x"a11ceb0b060000000b010012021220033250048201140596017e079402e50208f904800106f9057b0af4060e0c82077d0dff070200000101020202030204020503060307030800090c00000a0800030b0000040f0c0100010810080007110001080101170700000c000100000d020100051204050002130708010804140a08010808150c0d010806160e01010c0118101100071912010108051a01130100071b140d0108081c0215010c081616010108021d1701010803060409050b060b080b090b0a0b0b0b0c0b0d0602070802060c000107080202050b0501080001060c010501080102070802050107090001080401070b03010900010800020708040f010b0501090003070802050b05010900030b050108000b030108040c010a02010806040708020806080602010c020708020f010b03010804020708040b0501090003070802060c090022636f696e5f6d6f64756c655f6964656e7469666965725f706c616365686f6c64657206737472696e670f6163636f756e745f73746f7261676507636f6e74657874066f626a656374067369676e6572126163636f756e745f636f696e5f73746f726504636f696e0a636f696e5f73746f726522434f494e5f5354525543545f4944454e5449464945525f504c414345484f4c44455208547265617375727907436f6e746578740666617563657404696e69740b64756d6d795f6669656c64064f626a65637409436f696e53746f726504436f696e0a616464726573735f6f6611676c6f62616c5f626f72726f775f6d75740a626f72726f775f6d7574087769746864726177076465706f73697406537472696e6704757466380f72656769737465725f657874656e640d6d6f64756c655f7369676e65720b6d696e745f657874656e64116372656174655f636f696e5f73746f72650e676c6f62616c5f6d6f76655f746fdeadeadeadeadeadeadeadeadeadeadeadeadeadeadeadeadeadeadeadeadead0000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000030201de0f20800283b61c0000000000000000000000000000000000000000000000000000000520deadeadeadeadeadeadeadeadeadeadeadeadeadeadeadeadeadeadeadeadead0a021615434f494e5f4e414d455f504c414345484f4c4445520a021817434f494e5f53594d424f4c5f504c414345484f4c4445520002010e01010201080b030108040001040003100b0111020c020a00070238000f0038014a102700000000000000000000000000000000000000000000000000000000000038020c030b000b020b03380302010000000f1a0a0007031107070411070700380438050c030a00070138060c010a0038070c020d0238010b0138080b000e030b021201380902010000";

      let template_store = account_storage::global_borrow_mut<TemplateStore>(ctx, @rooch_examples);
      table::add(&mut template_store.templates, name, template_bytes);
   }

   public entry fun issue_fixed_supply_coin(ctx: &mut Context, account: &signer, 
      module_name: String, coin_name: String, 
      coin_symbol: String, total_supply: u256, decimals: u8
   ) {
      let template_store = account_storage::global_borrow_mut<TemplateStore>(ctx, @rooch_examples);
      let template_bytes = *table::borrow(&template_store.templates, string::utf8(b"fixed_supply_coin"));
      let template_module = move_module::new(template_bytes);

      let sender = signer::address_of(account);
      let modules = vector::singleton(template_module);
      let modules = move_module::binding_module_address(modules, TEMPLATE_MODULE_ADDRESS, sender);
      let modules = move_module::replace_module_identiner(
         modules, 
         vector::singleton(string::utf8(TEMPLATE_MODULE_IDENTIFIER)), 
         vector::singleton(module_name)
      );
      let modules = move_module::replace_struct_identifier(
         modules,
         vector::singleton(string::utf8(TEMPLATE_COIN_STRUCT_IDENTIFIER_PLACEHOLDER)),
         vector::singleton(coin_symbol)
      );

      let old_strings = vector::singleton(string::utf8(TEMPLATE_COIN_NAME_PLACEHOLDER));
      vector::push_back(&mut old_strings, string::utf8(TEMPLATE_COIN_SYMBOL_PLACEHOLDER));
      let new_strings = vector::singleton(coin_name);
      vector::push_back(&mut new_strings, coin_symbol);
      let modules = move_module::replace_constant_string(
         modules,
         old_strings,
         new_strings
      );

      let new_supply = vector::singleton(total_supply);
      let old_supply = vector::singleton(TEMPLATE_COIN_SUPPLY_PLACEHOLDER);
      let modules = move_module::replace_constant_u256(modules, old_supply, new_supply);

      let new_decimal = vector::singleton(decimals);
      let old_decimal = vector::singleton(TEMPLATE_COIN_DECIMALS);
      let modules = move_module::replace_constant_u8(modules, old_decimal, new_decimal);

      // publish modules
      account_storage::publish_modules(ctx, account, modules);
   }
}