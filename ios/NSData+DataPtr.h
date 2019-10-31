//
//  NSData+DataPtr.h
//  ChainLibs
//
//  Created by Ostap Danylovych on 24.10.2019.
//  Copyright © 2019 Facebook. All rights reserved.
//

#import <Foundation/Foundation.h>
#import <react_native_chain_libs.h>

NS_ASSUME_NONNULL_BEGIN

@interface NSData (DataPtr)

+ (NSData *)fromDataPtr:(DataPtr *)ptr;

+ (NSData *)fromBase64:(NSString *)base64Encoded;

- (NSString *)base64;

@end

NS_ASSUME_NONNULL_END
