CREATE OR REPLACE FUNCTION scm_order_contract_assignment_view_paged_with_filters(
    
        
    param_order_id_0_enabled boolean = FALSE,
    param_order_id_0_comparator character varying = '=',
    param_order_id_0_value integer = 0,
        
    param_order_number_0_enabled boolean = FALSE,
    param_order_number_0_comparator character varying = '=',
    param_order_number_0_value character varying = '',
        
    param_enrollment_id_0_enabled boolean = FALSE,
    param_enrollment_id_0_comparator character varying = '=',
    param_enrollment_id_0_value integer = 0,
        
    param_contract_id_0_enabled boolean = FALSE,
    param_contract_id_0_comparator character varying = '=',
    param_contract_id_0_value integer = 0,
        
    param_sku_name_0_enabled boolean = FALSE,
    param_sku_name_0_comparator character varying = '=',
    param_sku_name_0_value character varying = '',
        
    param_group_name_0_enabled boolean = FALSE,
    param_group_name_0_comparator character varying = '=',
    param_group_name_0_value character varying = '',
        
    param_contract_name_0_enabled boolean = FALSE,
    param_contract_name_0_comparator character varying = '=',
    param_contract_name_0_value character varying = '',
        
    param_recipient_name_0_enabled boolean = FALSE,
    param_recipient_name_0_comparator character varying = '=',
    param_recipient_name_0_value character varying = '',
        
    param_recipient_address_0_enabled boolean = FALSE,
    param_recipient_address_0_comparator character varying = '=',
    param_recipient_address_0_value character varying = '',
        
    param_quantity_0_enabled boolean = FALSE,
    param_quantity_0_comparator character varying = '=',
    param_quantity_0_value integer = 0,
        
    param_order_reason_0_enabled boolean = FALSE,
    param_order_reason_0_comparator character varying = '=',
    param_order_reason_0_value character varying = ''
        
        ,
    
        
    param_order_id_1_enabled boolean = FALSE,
    param_order_id_1_comparator character varying = '=',
    param_order_id_1_value integer = 0,
        
    param_order_number_1_enabled boolean = FALSE,
    param_order_number_1_comparator character varying = '=',
    param_order_number_1_value character varying = '',
        
    param_enrollment_id_1_enabled boolean = FALSE,
    param_enrollment_id_1_comparator character varying = '=',
    param_enrollment_id_1_value integer = 0,
        
    param_contract_id_1_enabled boolean = FALSE,
    param_contract_id_1_comparator character varying = '=',
    param_contract_id_1_value integer = 0,
        
    param_sku_name_1_enabled boolean = FALSE,
    param_sku_name_1_comparator character varying = '=',
    param_sku_name_1_value character varying = '',
        
    param_group_name_1_enabled boolean = FALSE,
    param_group_name_1_comparator character varying = '=',
    param_group_name_1_value character varying = '',
        
    param_contract_name_1_enabled boolean = FALSE,
    param_contract_name_1_comparator character varying = '=',
    param_contract_name_1_value character varying = '',
        
    param_recipient_name_1_enabled boolean = FALSE,
    param_recipient_name_1_comparator character varying = '=',
    param_recipient_name_1_value character varying = '',
        
    param_recipient_address_1_enabled boolean = FALSE,
    param_recipient_address_1_comparator character varying = '=',
    param_recipient_address_1_value character varying = '',
        
    param_quantity_1_enabled boolean = FALSE,
    param_quantity_1_comparator character varying = '=',
    param_quantity_1_value integer = 0,
        
    param_order_reason_1_enabled boolean = FALSE,
    param_order_reason_1_comparator character varying = '=',
    param_order_reason_1_value character varying = ''
        
        
    
    ,
    param_limit integer = 10000,
    param_offset integer = 0
) RETURNS TABLE (
    
            order_id character varying,
    
            order_number character varying,
    
            group_name character varying,
    
            contract_name character varying,
    
            recipient_name character varying,
    
            recipient_address character varying,
    
            enrollment_id character varying,
    
            contract_id character varying,
    
            quantity character varying,
    
            external_id character varying,
    
            sku_name character varying,
    
            event_list character varying,
    
            order_reason character varying
    
)
AS $$
BEGIN
    RETURN QUERY SELECT 
        
            v.order_id,
        
            v.order_number,
        
            v.group_name,
        
            v.contract_name,
        
            v.recipient_name,
        
            v.recipient_address,
        
            v.enrollment_id,
        
            v.contract_id,
        
            v.quantity,
        
            v.external_id,
        
            v.sku_name,
        
            v.event_list,
        
            v.order_reason
        
    FROM scm_order_contract_assignment_view v
    WHERE
    
    
    CASE
        WHEN param_order_id_0_enabled AND param_order_id_0_comparator = '<' THEN v.order_id < param_order_id_0_value
        WHEN param_order_id_0_enabled AND param_order_id_0_comparator = '<=' THEN v.order_id <= param_order_id_0_value
        WHEN param_order_id_0_enabled AND param_order_id_0_comparator = '>' THEN v.order_id > param_order_id_0_value
        WHEN param_order_id_0_enabled AND param_order_id_0_comparator = '>=' THEN v.order_id >= param_order_id_0_value
        WHEN param_order_id_0_enabled AND param_order_id_0_comparator = '=' THEN v.order_id = param_order_id_0_value
        WHEN param_order_id_0_enabled AND param_order_id_0_comparator = '<>' THEN v.order_id <> param_order_id_0_value 
        ELSE TRUE
    END
    AND
    
    CASE
        WHEN param_order_number_0_enabled AND param_order_number_0_comparator = '<' THEN v.order_number < param_order_number_0_value
        WHEN param_order_number_0_enabled AND param_order_number_0_comparator = '<=' THEN v.order_number <= param_order_number_0_value
        WHEN param_order_number_0_enabled AND param_order_number_0_comparator = '>' THEN v.order_number > param_order_number_0_value
        WHEN param_order_number_0_enabled AND param_order_number_0_comparator = '>=' THEN v.order_number >= param_order_number_0_value
        WHEN param_order_number_0_enabled AND param_order_number_0_comparator = '=' THEN v.order_number = param_order_number_0_value
        WHEN param_order_number_0_enabled AND param_order_number_0_comparator = '<>' THEN v.order_number <> param_order_number_0_value 
        ELSE TRUE
    END
    AND
    
    CASE
        WHEN param_enrollment_id_0_enabled AND param_enrollment_id_0_comparator = '<' THEN v.enrollment_id < param_enrollment_id_0_value
        WHEN param_enrollment_id_0_enabled AND param_enrollment_id_0_comparator = '<=' THEN v.enrollment_id <= param_enrollment_id_0_value
        WHEN param_enrollment_id_0_enabled AND param_enrollment_id_0_comparator = '>' THEN v.enrollment_id > param_enrollment_id_0_value
        WHEN param_enrollment_id_0_enabled AND param_enrollment_id_0_comparator = '>=' THEN v.enrollment_id >= param_enrollment_id_0_value
        WHEN param_enrollment_id_0_enabled AND param_enrollment_id_0_comparator = '=' THEN v.enrollment_id = param_enrollment_id_0_value
        WHEN param_enrollment_id_0_enabled AND param_enrollment_id_0_comparator = '<>' THEN v.enrollment_id <> param_enrollment_id_0_value 
        ELSE TRUE
    END
    AND
    
    CASE
        WHEN param_contract_id_0_enabled AND param_contract_id_0_comparator = '<' THEN v.contract_id < param_contract_id_0_value
        WHEN param_contract_id_0_enabled AND param_contract_id_0_comparator = '<=' THEN v.contract_id <= param_contract_id_0_value
        WHEN param_contract_id_0_enabled AND param_contract_id_0_comparator = '>' THEN v.contract_id > param_contract_id_0_value
        WHEN param_contract_id_0_enabled AND param_contract_id_0_comparator = '>=' THEN v.contract_id >= param_contract_id_0_value
        WHEN param_contract_id_0_enabled AND param_contract_id_0_comparator = '=' THEN v.contract_id = param_contract_id_0_value
        WHEN param_contract_id_0_enabled AND param_contract_id_0_comparator = '<>' THEN v.contract_id <> param_contract_id_0_value 
        ELSE TRUE
    END
    AND
    
    CASE
        WHEN param_sku_name_0_enabled AND param_sku_name_0_comparator = '<' THEN v.sku_name < param_sku_name_0_value
        WHEN param_sku_name_0_enabled AND param_sku_name_0_comparator = '<=' THEN v.sku_name <= param_sku_name_0_value
        WHEN param_sku_name_0_enabled AND param_sku_name_0_comparator = '>' THEN v.sku_name > param_sku_name_0_value
        WHEN param_sku_name_0_enabled AND param_sku_name_0_comparator = '>=' THEN v.sku_name >= param_sku_name_0_value
        WHEN param_sku_name_0_enabled AND param_sku_name_0_comparator = '=' THEN v.sku_name = param_sku_name_0_value
        WHEN param_sku_name_0_enabled AND param_sku_name_0_comparator = '<>' THEN v.sku_name <> param_sku_name_0_value 
        ELSE TRUE
    END
    AND
    
    CASE
        WHEN param_group_name_0_enabled AND param_group_name_0_comparator = '<' THEN v.group_name < param_group_name_0_value
        WHEN param_group_name_0_enabled AND param_group_name_0_comparator = '<=' THEN v.group_name <= param_group_name_0_value
        WHEN param_group_name_0_enabled AND param_group_name_0_comparator = '>' THEN v.group_name > param_group_name_0_value
        WHEN param_group_name_0_enabled AND param_group_name_0_comparator = '>=' THEN v.group_name >= param_group_name_0_value
        WHEN param_group_name_0_enabled AND param_group_name_0_comparator = '=' THEN v.group_name = param_group_name_0_value
        WHEN param_group_name_0_enabled AND param_group_name_0_comparator = '<>' THEN v.group_name <> param_group_name_0_value 
        ELSE TRUE
    END
    AND
    
    CASE
        WHEN param_contract_name_0_enabled AND param_contract_name_0_comparator = '<' THEN v.contract_name < param_contract_name_0_value
        WHEN param_contract_name_0_enabled AND param_contract_name_0_comparator = '<=' THEN v.contract_name <= param_contract_name_0_value
        WHEN param_contract_name_0_enabled AND param_contract_name_0_comparator = '>' THEN v.contract_name > param_contract_name_0_value
        WHEN param_contract_name_0_enabled AND param_contract_name_0_comparator = '>=' THEN v.contract_name >= param_contract_name_0_value
        WHEN param_contract_name_0_enabled AND param_contract_name_0_comparator = '=' THEN v.contract_name = param_contract_name_0_value
        WHEN param_contract_name_0_enabled AND param_contract_name_0_comparator = '<>' THEN v.contract_name <> param_contract_name_0_value 
        ELSE TRUE
    END
    AND
    
    CASE
        WHEN param_recipient_name_0_enabled AND param_recipient_name_0_comparator = '<' THEN v.recipient_name < param_recipient_name_0_value
        WHEN param_recipient_name_0_enabled AND param_recipient_name_0_comparator = '<=' THEN v.recipient_name <= param_recipient_name_0_value
        WHEN param_recipient_name_0_enabled AND param_recipient_name_0_comparator = '>' THEN v.recipient_name > param_recipient_name_0_value
        WHEN param_recipient_name_0_enabled AND param_recipient_name_0_comparator = '>=' THEN v.recipient_name >= param_recipient_name_0_value
        WHEN param_recipient_name_0_enabled AND param_recipient_name_0_comparator = '=' THEN v.recipient_name = param_recipient_name_0_value
        WHEN param_recipient_name_0_enabled AND param_recipient_name_0_comparator = '<>' THEN v.recipient_name <> param_recipient_name_0_value 
        ELSE TRUE
    END
    AND
    
    CASE
        WHEN param_recipient_address_0_enabled AND param_recipient_address_0_comparator = '<' THEN v.recipient_address < param_recipient_address_0_value
        WHEN param_recipient_address_0_enabled AND param_recipient_address_0_comparator = '<=' THEN v.recipient_address <= param_recipient_address_0_value
        WHEN param_recipient_address_0_enabled AND param_recipient_address_0_comparator = '>' THEN v.recipient_address > param_recipient_address_0_value
        WHEN param_recipient_address_0_enabled AND param_recipient_address_0_comparator = '>=' THEN v.recipient_address >= param_recipient_address_0_value
        WHEN param_recipient_address_0_enabled AND param_recipient_address_0_comparator = '=' THEN v.recipient_address = param_recipient_address_0_value
        WHEN param_recipient_address_0_enabled AND param_recipient_address_0_comparator = '<>' THEN v.recipient_address <> param_recipient_address_0_value 
        ELSE TRUE
    END
    AND
    
    CASE
        WHEN param_quantity_0_enabled AND param_quantity_0_comparator = '<' THEN v.quantity < param_quantity_0_value
        WHEN param_quantity_0_enabled AND param_quantity_0_comparator = '<=' THEN v.quantity <= param_quantity_0_value
        WHEN param_quantity_0_enabled AND param_quantity_0_comparator = '>' THEN v.quantity > param_quantity_0_value
        WHEN param_quantity_0_enabled AND param_quantity_0_comparator = '>=' THEN v.quantity >= param_quantity_0_value
        WHEN param_quantity_0_enabled AND param_quantity_0_comparator = '=' THEN v.quantity = param_quantity_0_value
        WHEN param_quantity_0_enabled AND param_quantity_0_comparator = '<>' THEN v.quantity <> param_quantity_0_value 
        ELSE TRUE
    END
    AND
    
    CASE
        WHEN param_order_reason_0_enabled AND param_order_reason_0_comparator = '<' THEN v.order_reason < param_order_reason_0_value
        WHEN param_order_reason_0_enabled AND param_order_reason_0_comparator = '<=' THEN v.order_reason <= param_order_reason_0_value
        WHEN param_order_reason_0_enabled AND param_order_reason_0_comparator = '>' THEN v.order_reason > param_order_reason_0_value
        WHEN param_order_reason_0_enabled AND param_order_reason_0_comparator = '>=' THEN v.order_reason >= param_order_reason_0_value
        WHEN param_order_reason_0_enabled AND param_order_reason_0_comparator = '=' THEN v.order_reason = param_order_reason_0_value
        WHEN param_order_reason_0_enabled AND param_order_reason_0_comparator = '<>' THEN v.order_reason <> param_order_reason_0_value 
        ELSE TRUE
    END
    
    
    AND
    
    
    CASE
        WHEN param_order_id_1_enabled AND param_order_id_1_comparator = '<' THEN v.order_id < param_order_id_1_value
        WHEN param_order_id_1_enabled AND param_order_id_1_comparator = '<=' THEN v.order_id <= param_order_id_1_value
        WHEN param_order_id_1_enabled AND param_order_id_1_comparator = '>' THEN v.order_id > param_order_id_1_value
        WHEN param_order_id_1_enabled AND param_order_id_1_comparator = '>=' THEN v.order_id >= param_order_id_1_value
        WHEN param_order_id_1_enabled AND param_order_id_1_comparator = '=' THEN v.order_id = param_order_id_1_value
        WHEN param_order_id_1_enabled AND param_order_id_1_comparator = '<>' THEN v.order_id <> param_order_id_1_value 
        ELSE TRUE
    END
    AND
    
    CASE
        WHEN param_order_number_1_enabled AND param_order_number_1_comparator = '<' THEN v.order_number < param_order_number_1_value
        WHEN param_order_number_1_enabled AND param_order_number_1_comparator = '<=' THEN v.order_number <= param_order_number_1_value
        WHEN param_order_number_1_enabled AND param_order_number_1_comparator = '>' THEN v.order_number > param_order_number_1_value
        WHEN param_order_number_1_enabled AND param_order_number_1_comparator = '>=' THEN v.order_number >= param_order_number_1_value
        WHEN param_order_number_1_enabled AND param_order_number_1_comparator = '=' THEN v.order_number = param_order_number_1_value
        WHEN param_order_number_1_enabled AND param_order_number_1_comparator = '<>' THEN v.order_number <> param_order_number_1_value 
        ELSE TRUE
    END
    AND
    
    CASE
        WHEN param_enrollment_id_1_enabled AND param_enrollment_id_1_comparator = '<' THEN v.enrollment_id < param_enrollment_id_1_value
        WHEN param_enrollment_id_1_enabled AND param_enrollment_id_1_comparator = '<=' THEN v.enrollment_id <= param_enrollment_id_1_value
        WHEN param_enrollment_id_1_enabled AND param_enrollment_id_1_comparator = '>' THEN v.enrollment_id > param_enrollment_id_1_value
        WHEN param_enrollment_id_1_enabled AND param_enrollment_id_1_comparator = '>=' THEN v.enrollment_id >= param_enrollment_id_1_value
        WHEN param_enrollment_id_1_enabled AND param_enrollment_id_1_comparator = '=' THEN v.enrollment_id = param_enrollment_id_1_value
        WHEN param_enrollment_id_1_enabled AND param_enrollment_id_1_comparator = '<>' THEN v.enrollment_id <> param_enrollment_id_1_value 
        ELSE TRUE
    END
    AND
    
    CASE
        WHEN param_contract_id_1_enabled AND param_contract_id_1_comparator = '<' THEN v.contract_id < param_contract_id_1_value
        WHEN param_contract_id_1_enabled AND param_contract_id_1_comparator = '<=' THEN v.contract_id <= param_contract_id_1_value
        WHEN param_contract_id_1_enabled AND param_contract_id_1_comparator = '>' THEN v.contract_id > param_contract_id_1_value
        WHEN param_contract_id_1_enabled AND param_contract_id_1_comparator = '>=' THEN v.contract_id >= param_contract_id_1_value
        WHEN param_contract_id_1_enabled AND param_contract_id_1_comparator = '=' THEN v.contract_id = param_contract_id_1_value
        WHEN param_contract_id_1_enabled AND param_contract_id_1_comparator = '<>' THEN v.contract_id <> param_contract_id_1_value 
        ELSE TRUE
    END
    AND
    
    CASE
        WHEN param_sku_name_1_enabled AND param_sku_name_1_comparator = '<' THEN v.sku_name < param_sku_name_1_value
        WHEN param_sku_name_1_enabled AND param_sku_name_1_comparator = '<=' THEN v.sku_name <= param_sku_name_1_value
        WHEN param_sku_name_1_enabled AND param_sku_name_1_comparator = '>' THEN v.sku_name > param_sku_name_1_value
        WHEN param_sku_name_1_enabled AND param_sku_name_1_comparator = '>=' THEN v.sku_name >= param_sku_name_1_value
        WHEN param_sku_name_1_enabled AND param_sku_name_1_comparator = '=' THEN v.sku_name = param_sku_name_1_value
        WHEN param_sku_name_1_enabled AND param_sku_name_1_comparator = '<>' THEN v.sku_name <> param_sku_name_1_value 
        ELSE TRUE
    END
    AND
    
    CASE
        WHEN param_group_name_1_enabled AND param_group_name_1_comparator = '<' THEN v.group_name < param_group_name_1_value
        WHEN param_group_name_1_enabled AND param_group_name_1_comparator = '<=' THEN v.group_name <= param_group_name_1_value
        WHEN param_group_name_1_enabled AND param_group_name_1_comparator = '>' THEN v.group_name > param_group_name_1_value
        WHEN param_group_name_1_enabled AND param_group_name_1_comparator = '>=' THEN v.group_name >= param_group_name_1_value
        WHEN param_group_name_1_enabled AND param_group_name_1_comparator = '=' THEN v.group_name = param_group_name_1_value
        WHEN param_group_name_1_enabled AND param_group_name_1_comparator = '<>' THEN v.group_name <> param_group_name_1_value 
        ELSE TRUE
    END
    AND
    
    CASE
        WHEN param_contract_name_1_enabled AND param_contract_name_1_comparator = '<' THEN v.contract_name < param_contract_name_1_value
        WHEN param_contract_name_1_enabled AND param_contract_name_1_comparator = '<=' THEN v.contract_name <= param_contract_name_1_value
        WHEN param_contract_name_1_enabled AND param_contract_name_1_comparator = '>' THEN v.contract_name > param_contract_name_1_value
        WHEN param_contract_name_1_enabled AND param_contract_name_1_comparator = '>=' THEN v.contract_name >= param_contract_name_1_value
        WHEN param_contract_name_1_enabled AND param_contract_name_1_comparator = '=' THEN v.contract_name = param_contract_name_1_value
        WHEN param_contract_name_1_enabled AND param_contract_name_1_comparator = '<>' THEN v.contract_name <> param_contract_name_1_value 
        ELSE TRUE
    END
    AND
    
    CASE
        WHEN param_recipient_name_1_enabled AND param_recipient_name_1_comparator = '<' THEN v.recipient_name < param_recipient_name_1_value
        WHEN param_recipient_name_1_enabled AND param_recipient_name_1_comparator = '<=' THEN v.recipient_name <= param_recipient_name_1_value
        WHEN param_recipient_name_1_enabled AND param_recipient_name_1_comparator = '>' THEN v.recipient_name > param_recipient_name_1_value
        WHEN param_recipient_name_1_enabled AND param_recipient_name_1_comparator = '>=' THEN v.recipient_name >= param_recipient_name_1_value
        WHEN param_recipient_name_1_enabled AND param_recipient_name_1_comparator = '=' THEN v.recipient_name = param_recipient_name_1_value
        WHEN param_recipient_name_1_enabled AND param_recipient_name_1_comparator = '<>' THEN v.recipient_name <> param_recipient_name_1_value 
        ELSE TRUE
    END
    AND
    
    CASE
        WHEN param_recipient_address_1_enabled AND param_recipient_address_1_comparator = '<' THEN v.recipient_address < param_recipient_address_1_value
        WHEN param_recipient_address_1_enabled AND param_recipient_address_1_comparator = '<=' THEN v.recipient_address <= param_recipient_address_1_value
        WHEN param_recipient_address_1_enabled AND param_recipient_address_1_comparator = '>' THEN v.recipient_address > param_recipient_address_1_value
        WHEN param_recipient_address_1_enabled AND param_recipient_address_1_comparator = '>=' THEN v.recipient_address >= param_recipient_address_1_value
        WHEN param_recipient_address_1_enabled AND param_recipient_address_1_comparator = '=' THEN v.recipient_address = param_recipient_address_1_value
        WHEN param_recipient_address_1_enabled AND param_recipient_address_1_comparator = '<>' THEN v.recipient_address <> param_recipient_address_1_value 
        ELSE TRUE
    END
    AND
    
    CASE
        WHEN param_quantity_1_enabled AND param_quantity_1_comparator = '<' THEN v.quantity < param_quantity_1_value
        WHEN param_quantity_1_enabled AND param_quantity_1_comparator = '<=' THEN v.quantity <= param_quantity_1_value
        WHEN param_quantity_1_enabled AND param_quantity_1_comparator = '>' THEN v.quantity > param_quantity_1_value
        WHEN param_quantity_1_enabled AND param_quantity_1_comparator = '>=' THEN v.quantity >= param_quantity_1_value
        WHEN param_quantity_1_enabled AND param_quantity_1_comparator = '=' THEN v.quantity = param_quantity_1_value
        WHEN param_quantity_1_enabled AND param_quantity_1_comparator = '<>' THEN v.quantity <> param_quantity_1_value 
        ELSE TRUE
    END
    AND
    
    CASE
        WHEN param_order_reason_1_enabled AND param_order_reason_1_comparator = '<' THEN v.order_reason < param_order_reason_1_value
        WHEN param_order_reason_1_enabled AND param_order_reason_1_comparator = '<=' THEN v.order_reason <= param_order_reason_1_value
        WHEN param_order_reason_1_enabled AND param_order_reason_1_comparator = '>' THEN v.order_reason > param_order_reason_1_value
        WHEN param_order_reason_1_enabled AND param_order_reason_1_comparator = '>=' THEN v.order_reason >= param_order_reason_1_value
        WHEN param_order_reason_1_enabled AND param_order_reason_1_comparator = '=' THEN v.order_reason = param_order_reason_1_value
        WHEN param_order_reason_1_enabled AND param_order_reason_1_comparator = '<>' THEN v.order_reason <> param_order_reason_1_value 
        ELSE TRUE
    END
    
    
    
    ;
END;
$$ LANGUAGE plpgsql;