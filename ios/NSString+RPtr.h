//
//  NSString+RPtr.h
//  ChainLibs
//
//  Created by Yehor Popovych on 10/24/19.
//  Copyright © 2019 Facebook. All rights reserved.
//

#import <Foundation/Foundation.h>
#import <react_native_chain_libs.h>

NS_ASSUME_NONNULL_BEGIN

@interface NSString (RPtr)

+ (NSString *)stringFromPtr:(RPtr)ptr;

+ (NSString *)stringFromCharPtr:(CharPtr _Nonnull * _Nonnull)ptr;

- (CharPtr)charPtr;

- (RPtr)rPtr;

@end

NS_ASSUME_NONNULL_END
