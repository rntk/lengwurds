// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'main.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Lang _$LangFromJson(Map<String, dynamic> json) => Lang(
      lang: json['lang'] as String,
    );

Map<String, dynamic> _$LangToJson(Lang instance) => <String, dynamic>{
      'lang': instance.lang,
    };

Word _$WordFromJson(Map<String, dynamic> json) => Word(
      word: json['word'] as String,
      lang: Lang.fromJson(json['lang'] as Map<String, dynamic>),
    );

Map<String, dynamic> _$WordToJson(Word instance) => <String, dynamic>{
      'lang': instance.lang,
      'word': instance.word,
    };

Translate _$TranslateFromJson(Map<String, dynamic> json) => Translate(
      word: Word.fromJson(json['word'] as Map<String, dynamic>),
      translates: (json['translates'] as List<dynamic>)
          .map((e) => Word.fromJson(e as Map<String, dynamic>))
          .toList(),
    );

Map<String, dynamic> _$TranslateToJson(Translate instance) => <String, dynamic>{
      'word': instance.word,
      'translates': instance.translates,
    };
